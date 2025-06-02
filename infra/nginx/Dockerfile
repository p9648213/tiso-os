# ---- Build Stage ----
    FROM debian:bullseye AS builder

    # Install build dependencies only
    RUN apt-get update && apt-get install -y \
        wget \
        git \
        build-essential \
        libpcre3-dev \
        zlib1g-dev \
        libzstd-dev \
        ca-certificates \
        && apt-get clean && rm -rf /var/lib/apt/lists/*
    
    WORKDIR /src
    
    # Download and extract nginx
    RUN wget https://nginx.org/download/nginx-1.28.0.tar.gz && \
        tar -xzf nginx-1.28.0.tar.gz
    
    # Clone zstd module
    RUN git clone --depth=1 https://github.com/tokers/zstd-nginx-module.git
    
    # Configure and build nginx
    WORKDIR /src/nginx-1.28.0
    RUN ./configure \
        --add-module=../zstd-nginx-module \
        --with-http_gzip_static_module \
        --prefix=/etc/nginx \
        --sbin-path=/usr/sbin/nginx \
        --conf-path=/etc/nginx/nginx.conf \
        --error-log-path=/var/log/nginx/error.log \
        --http-log-path=/var/log/nginx/access.log \
        --pid-path=/var/run/nginx.pid \
        --lock-path=/var/run/nginx.lock \
        --user=nginx \
        --group=nginx \
        && make -j$(nproc) && make install
    
    # Copy only needed files
    RUN cp conf/mime.types /etc/nginx/mime.types
    
    # ---- Runtime Stage ----
    FROM debian:bullseye-slim
    
    # Install runtime dependencies
    RUN apt-get update && apt-get install -y \
        libpcre3 \
        zlib1g \
        libzstd1 \
        && apt-get clean && rm -rf /var/lib/apt/lists/*
    
    # Create nginx user and minimal dirs
    RUN addgroup --system nginx && \
        adduser --system --ingroup nginx --no-create-home --shell /bin/false nginx && \
        mkdir -p /var/log/nginx /var/run /usr/share/nginx/assets && \
        chown -R nginx:nginx /var/log/nginx /usr/share/nginx/assets
    
    # Copy nginx binary and config
    COPY --from=builder /usr/sbin/nginx /usr/sbin/nginx
    COPY --from=builder /etc/nginx/mime.types /etc/nginx/mime.types
    
    # Add healthcheck (optional)
    HEALTHCHECK CMD curl --fail http://localhost/ || exit 1
    
    CMD ["nginx", "-g", "daemon off;"]
    