;;不显示工具栏

;; Added by Package.el.  This must come before configurations of
;; installed packages.  Don't delete this line.  If you don't want it,
;; just comment it out by adding a semicolon to the start of the line.
;; You may delete these explanatory comments.
(package-initialize)

(tool-bar-mode 0)
;;不显示菜单栏
(menu-bar-mode 0)
;;不显示滚动条
(scroll-bar-mode 0)
;;显示行号与列号
(global-linum-mode 1)
(column-number-mode 1)
;;更改光标的样式
(setq-default cursor-type 'bar)
(blink-cursor-mode 0)
;;关闭启动动画
(setq inhibit-startup-message 1)
;;高亮当前行
(global-hl-line-mode 1)
;;关闭buffer
(setq initial-scratch-message "")
;;设置字体
(set-frame-font "Fira Sans 18" nil t)
;;Start maximised
(add-hook 'window-setup-hook 'toggle-frame-maximized t)
;;Start fullscreen
(add-hook 'window-setup-hook 'toggle-frame-fullscreen t)
;;关闭默认的哔哔提示音
(setq ring-bell-function 'ignore)
;;设置问答提示为 y-or-n,而不是yes-or-no
(fset 'yes-or-no-p 'y-or-n-p)
;;不生成备份文件，即 xxx.xx~ 类文件
(setq make-backup-files nil)

;;https://github.com/melpa/melpa
(require 'package)
(add-to-list 'package-archives '("melpa" . "https://melpa.org/packages/") t)
(package-initialize)

;;https://draculatheme.com/emacs/
(custom-set-variables
 ;; custom-set-variables was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(package-selected-packages (quote (rust-mode python-mode dracula-theme))))
(custom-set-faces
 ;; custom-set-faces was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 )
(load-theme 'dracula t)
