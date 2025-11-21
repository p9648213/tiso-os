export const MessageBox = {
  icons: {
    info: "/assets/images/message-box/info.svg",
    error: "/assets/images/message-box/error.svg",
    warning: "/assets/images/message-box/warning.svg",
  },

  _show(type, title, message) {
    return new Promise((resolve) => {
      const existingBoxes = document.querySelectorAll('.js-message-box');
      const offsetCount = existingBoxes.length;
      const offsetPx = offsetCount * 20;
      
      const box = document.createElement("div");

      box.className = "js-message-box fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white w-[400px] max-w-[90%] rounded-sm shadow-2xl border border-zinc-300 flex flex-col overflow-hidden font-sans";

      box.style.marginTop = `${offsetPx}px`;
      box.style.marginLeft = `${offsetPx}px`;
      box.style.zIndex = `${50 + offsetCount}`;

      const header = document.createElement("div");
      header.className = "p-3 flex justify-between items-center";
      
      const titleEl = document.createElement("span");
      titleEl.className = "text-lg font-semibold text-zinc-900";
      titleEl.innerText = title;
      header.appendChild(titleEl);

      const body = document.createElement("div");
      body.className = "px-4 pb-6 flex flex-row items-start gap-4";
      
      const iconImg = document.createElement("img");
      iconImg.className = "w-10 h-10 object-contain shrink-0";
      iconImg.src = this.icons[type] || this.icons.info;
      
      const contentEl = document.createElement("div");
      contentEl.className = "text-zinc-800 pt-1 leading-normal";
      contentEl.innerText = message;

      body.appendChild(iconImg);
      body.appendChild(contentEl);

      const footer = document.createElement("div");
      footer.className = "bg-zinc-50 px-4 py-2 flex justify-end gap-2 border-t border-zinc-200";

      const createBtn = (text, isBlueButton) => {
        const btn = document.createElement("button");
        
        let baseClass = "min-w-[80px] px-6 py-1.5 rounded text-sm transition-colors border outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1";
        
        if (isBlueButton) {
          btn.className = `${baseClass} bg-blue-600 text-white border-transparent hover:bg-blue-700`;
        } else {
          btn.className = `${baseClass} bg-white text-zinc-900 border-zinc-300 hover:bg-zinc-100`;
        }

        btn.innerText = text;
        
        btn.onclick = () => {
          document.body.removeChild(box);
          
          if (type === "warning") {
            resolve(isBlueButton);
          } else {
            resolve(true);
          }
        };
        return btn;
      };

      if (type === "warning") {
        const btnYes = createBtn("Yes", true);
        const btnNo = createBtn("No", false);
        footer.appendChild(btnYes);
        footer.appendChild(btnNo);
      } else {
        const btnOk = createBtn("OK", true);
        footer.appendChild(btnOk);
      }

      box.appendChild(header);
      box.appendChild(body);
      box.appendChild(footer);
      
      document.body.appendChild(box);
    });
  },

  async info(title, message) {
    return this._show("info", title, message);
  },

  async error(title, message) {
    return this._show("error", title, message);
  },

  async warning(title, message) {
    return this._show("warning", title, message);
  },
};