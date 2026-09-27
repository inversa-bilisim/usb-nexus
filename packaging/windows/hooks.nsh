; SPDX-License-Identifier: GPL-3.0-or-later
; NSIS hooks for the USB Nexus installer (Tauri). The service executable is
; bundled as a sidecar and installed next to the app as usbnexus.exe.
;
; Included near the top of installer.nsi (before its variables and
; languages): only defines and functions that need neither go here. The
; pages and everything using their choices live in usbnexus-pages.nsh, and the
; translated texts in usbnexus-strings.nsh (generated from locales/*.ftl).

!define USBNEXUS_DIR "${__FILEDIR__}"

; usbip-win2 (BSD-2-Clause) attaches remote devices on Windows. Its installer
; is bundled (see fetch-usbip-win2.ps1) and installed for the client role
; when missing or older than the oldest version whose usbip.exe has the
; options we use (`attach --once`, added in 0.9.7.6).
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

; Sets $R0 to what the client role needs: 0 nothing, 1 install usbip-win2,
; 2 update it.
Function usbnexus_usbip_win2_need
  Call usbnexus_usbip_win2_version
  ${If} $R0 == ""
    StrCpy $R0 1
  ${Else}
    ${VersionCompare} $R0 "${USBIP_WIN2_MIN_VERSION}" $R1
    ; 2 = the installed version is older than the minimum.
    ${If} $R1 == 2
      StrCpy $R0 2
    ${Else}
      StrCpy $R0 0
    ${EndIf}
  ${EndIf}
FunctionEnd

; Installs the bundled usbip-win2 silently (the role page told the user).
Function usbnexus_usbip_win2_install
  ${IfNot} ${FileExists} "${USBIP_WIN2_SETUP}"
    DetailPrint "usbip-win2 setup is not bundled; skipping."
    Return
  ${EndIf}
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
    DetailPrint "usbip-win2 setup failed with exit code $R1."
    MessageBox MB_OK|MB_ICONEXCLAMATION "$(setup_usbip_failed)" /SD IDOK
  ${EndIf}
FunctionEnd

!macro NSIS_HOOK_PREINSTALL
  ; An earlier version's service keeps usbnexus.exe open; stop it so the file
  ; can be replaced. `service install` below updates and restarts it.
  nsExec::ExecToLog 'sc.exe stop usbnexus'
  Sleep 3000
  ; The service loads the VBoxUSBMon kernel driver straight from
  ; $INSTDIR\drivers, which keeps VBoxUSBMon.sys locked. With our service
  ; stopped nobody holds it open, so it can be unloaded (this fails
  ; harmlessly while another program such as usbipd-win still uses it).
  nsExec::ExecToLog 'sc.exe stop VBoxUSBMon'
  Sleep 2000
!macroend

!macro NSIS_HOOK_POSTINSTALL
  ; Service, roles, web interface and usbip-win2 (usbnexus-pages.nsh).
  Call usbnexus_Configure
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  nsExec::ExecToLog '"$INSTDIR\usbnexus.exe" service uninstall'
  ; usbip-win2 stays installed: it may be used by other programs and has its
  ; own entry in "Installed apps".
!macroend
