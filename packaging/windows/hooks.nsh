; SPDX-License-Identifier: GPL-3.0-or-later
; NSIS hooks for the USB Nexus installer (Tauri). The service executable is
; bundled as a sidecar and installed next to the app as usbnexus.exe.

!macro NSIS_HOOK_POSTINSTALL
  ; Register and start the background service (runs as LocalSystem).
  nsExec::ExecToLog '"$INSTDIR\usbnexus.exe" service install'

  ; Remote devices need the signed usbip-win2 driver.
  ${IfNot} ${FileExists} "$PROGRAMFILES64\USBip\usbip.exe"
    StrCpy $0 "To use remote USB devices on this computer, the usbip-win2 driver is required.$\r$\n$\r$\nOpen the download page?"
    ; 1055 = Turkish
    ${If} $LANGUAGE == 1055
      StrCpy $0 "Uzak USB cihazlarını bu bilgisayarda kullanmak için usbip-win2 sürücüsü gerekir.$\r$\n$\r$\nİndirme sayfası açılsın mı?"
    ${EndIf}
    ; Silent installs (/S) skip the question.
    MessageBox MB_YESNO|MB_ICONINFORMATION "$0" /SD IDNO IDNO usbnexus_skip_driver
    ExecShell "open" "https://github.com/vadimgrn/usbip-win2/releases/latest"
    usbnexus_skip_driver:
  ${EndIf}
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  nsExec::ExecToLog '"$INSTDIR\usbnexus.exe" service uninstall'
!macroend
