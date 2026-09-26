; SPDX-License-Identifier: GPL-3.0-or-later
; NSIS hooks for the USB Nexus installer (Tauri). The service executable is
; bundled as a sidecar and installed next to the app as usbnexus.exe.

; usbip-win2 (BSD-2-Clause) attaches remote devices on Windows. Its installer
; is bundled (see fetch-usbip-win2.ps1) and offered when it is missing or
; older than the oldest version whose usbip.exe has the options we use
; (`attach --once`, added in 0.9.7.6).
!define USBIP_WIN2_MIN_VERSION "0.9.7.6"
!define USBIP_WIN2_SETUP "$INSTDIR\usbip-win2\usbip-win2-setup.exe"
; Inno Setup uninstall key (usbip-win2's AppId).
!define USBIP_WIN2_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\{199505b0-b93d-4521-a8c7-897818e0205a}_is1"

; Sets $R0 to the installed usbip-win2 version, or "" if it is not installed.
Function usbnexus_usbip_win2_version
  SetRegView 64
  ReadRegStr $R0 HKLM "${USBIP_WIN2_UNINST_KEY}" "DisplayVersion"
  SetRegView lastused
  ${If} $R0 == ""
  ${AndIf} ${FileExists} "$PROGRAMFILES64\USBip\usbip.exe"
    ; Installed without the uninstall entry (e.g. copied by hand): use the
    ; file version, or assume it is too old if there is none.
    ClearErrors
    GetDLLVersion "$PROGRAMFILES64\USBip\usbip.exe" $R1 $R2
    ${If} ${Errors}
      StrCpy $R0 "0"
    ${Else}
      IntOp $R3 $R1 >> 16
      IntOp $R4 $R1 & 0xFFFF
      IntOp $R5 $R2 >> 16
      IntOp $R6 $R2 & 0xFFFF
      StrCpy $R0 "$R3.$R4.$R5.$R6"
    ${EndIf}
  ${EndIf}
FunctionEnd

Function usbnexus_usbip_win2
  Call usbnexus_usbip_win2_version
  ${If} $R0 != ""
    ${VersionCompare} $R0 "${USBIP_WIN2_MIN_VERSION}" $R1
    ; 2 = the installed version is older than the minimum.
    ${If} $R1 != 2
      DetailPrint "usbip-win2 $R0 is already installed."
      Return
    ${EndIf}
    StrCpy $0 "usbip-win2 $R0 is installed on this computer, but USB Nexus needs version ${USBIP_WIN2_MIN_VERSION} or later to use remote USB devices.$\r$\n$\r$\nUpdate it with the version included in this setup?"
    ; 1055 = Turkish
    ${If} $LANGUAGE == 1055
      StrCpy $0 "Bu bilgisayarda usbip-win2 $R0 kurulu, ancak USB Nexus uzak USB cihazlarını kullanmak için ${USBIP_WIN2_MIN_VERSION} veya daha yeni bir sürüm gerektirir.$\r$\n$\r$\nBu kurulumla gelen sürüme güncellensin mi?"
    ${EndIf}
  ${Else}
    StrCpy $0 "To use USB devices of other computers on this computer, the usbip-win2 driver is required. It is included in this setup.$\r$\n$\r$\nInstall it now?"
    ${If} $LANGUAGE == 1055
      StrCpy $0 "Başka bilgisayarlardaki USB cihazlarını bu bilgisayarda kullanmak için usbip-win2 sürücüsü gerekir. Sürücü bu kurulumla birlikte geliyor.$\r$\n$\r$\nŞimdi kurulsun mu?"
    ${EndIf}
  ${EndIf}

  ${IfNot} ${FileExists} "${USBIP_WIN2_SETUP}"
    DetailPrint "usbip-win2 setup is not bundled; skipping."
    Return
  ${EndIf}

  StrCpy $1 "$\r$\n$\r$\nUSB devices plugged into this computer stop for a few seconds during the installation, and Windows must be restarted afterwards."
  ${If} $LANGUAGE == 1055
    StrCpy $1 "$\r$\n$\r$\nKurulum sırasında bu bilgisayara takılı USB cihazları birkaç saniye çalışmaz; sonrasında Windows'un yeniden başlatılması gerekir."
  ${EndIf}
  ; Silent installs (/S) skip it: restarting the USB hubs must be a choice.
  MessageBox MB_YESNO|MB_ICONQUESTION "$0$1" /SD IDNO IDYES usbnexus_usbip_win2_install
  Return

  usbnexus_usbip_win2_install:
  DetailPrint "Installing usbip-win2..."
  ClearErrors
  ExecWait '"${USBIP_WIN2_SETUP}" /SILENT /SUPPRESSMSGBOXES /NOCANCEL /SP- /NORESTART /RESTARTEXITCODE=3010 /CLOSEAPPLICATIONS /COMPONENTS="main,client" /TASKS="vcredist"' $R1
  ${If} ${Errors}
    StrCpy $R1 "?"
  ${EndIf}
  ${If} $R1 == 3010
    DetailPrint "usbip-win2 installed; Windows must be restarted."
    SetRebootFlag true
  ${ElseIf} $R1 == 0
    DetailPrint "usbip-win2 installed."
  ${Else}
    StrCpy $0 "usbip-win2 could not be installed (exit code $R1). You can run the USB Nexus setup again later to retry."
    ${If} $LANGUAGE == 1055
      StrCpy $0 "usbip-win2 kurulamadı (çıkış kodu $R1). Daha sonra USB Nexus kurulumunu yeniden çalıştırarak tekrar deneyebilirsiniz."
    ${EndIf}
    MessageBox MB_OK|MB_ICONEXCLAMATION "$0" /SD IDOK
  ${EndIf}
FunctionEnd

!macro NSIS_HOOK_POSTINSTALL
  ; Register and start the background service (runs as LocalSystem).
  nsExec::ExecToLog '"$INSTDIR\usbnexus.exe" service install'

  ; Remote devices need the signed usbip-win2 driver.
  Call usbnexus_usbip_win2
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  nsExec::ExecToLog '"$INSTDIR\usbnexus.exe" service uninstall'
  ; usbip-win2 stays installed: it may be used by other programs and has its
  ; own entry in "Installed apps".
!macroend
