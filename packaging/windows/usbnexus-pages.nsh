; SPDX-License-Identifier: GPL-3.0-or-later
; USB Nexus installer pages: roles (server / client / web access) and web
; interface settings, and applying the choices after the files are copied.
;
; Included by installer.nsi after its languages (so usbnexus-strings.nsh can use
; them) and its variables ($PassiveMode). Texts come from usbnexus-strings.nsh.

!include nsDialogs.nsh
!include LogicLib.nsh
!include WinMessages.nsh

; Choices of the last installation, preselected on upgrades.
!define USBNEXUS_SETUP_KEY "Software\USB Nexus\Setup"
!define USBNEXUS_WEB_DEFAULT_PORT 3242

; Choices: 1 = yes, 0 = no.
Var UN_Server
Var UN_Client
Var UN_Web
Var UN_WebLan
Var UN_WebPort
Var UN_WebPw
; Whether the choices were loaded, and whether the pages were shown (not in
; silent or passive mode; then the web settings are left as they are).
Var UN_Loaded
Var UN_PagesShown
; What the client role needs (see usbnexus_usbip_win2_need).
Var UN_UsbipNeed
; Whether a web password is already set (upgrades may keep it).
Var UN_HasPw
Var UN_PortOk

Var UN_ServerBox
Var UN_ClientBox
Var UN_WebBox
Var UN_Note
Var UN_LocalRadio
Var UN_NetRadio
Var UN_PortBox
Var UN_PortStatus
Var UN_Pw1
Var UN_Pw2

; Reads a remembered choice into $0, or `default` if there is none.
!macro usbnexus_ReadChoice NAME DEFAULT
  ReadRegStr $0 HKLM "${USBNEXUS_SETUP_KEY}" "${NAME}"
  ${IfThen} $0 == "" ${|} StrCpy $0 "${DEFAULT}" ${|}
!macroend

Function usbnexus_LoadChoices
  ${IfThen} $UN_Loaded == 1 ${|} Return ${|}
  StrCpy $UN_Loaded 1
  !insertmacro usbnexus_ReadChoice Server 1
  StrCpy $UN_Server $0
  !insertmacro usbnexus_ReadChoice Client 1
  StrCpy $UN_Client $0
  !insertmacro usbnexus_ReadChoice Web 1
  StrCpy $UN_Web $0
  !insertmacro usbnexus_ReadChoice WebLan 0
  StrCpy $UN_WebLan $0
  !insertmacro usbnexus_ReadChoice WebPort ${USBNEXUS_WEB_DEFAULT_PORT}
  StrCpy $UN_WebPort $0

  Call usbnexus_usbip_win2_need
  StrCpy $UN_UsbipNeed $R0

  ; The service executable answers questions before it is installed.
  Call usbnexus_ExtractTool
  nsExec::Exec '"$PLUGINSDIR\usbnexus.exe" web has-password'
  Pop $0
  ${If} $0 == 0
    StrCpy $UN_HasPw 1
  ${Else}
    StrCpy $UN_HasPw 0
  ${EndIf}
FunctionEnd

; ---------------------------------------------------------------- roles page

Function usbnexus_RolesPage
  ${IfThen} $PassiveMode = 1 ${|} Abort ${|}
  Call usbnexus_LoadChoices
  !insertmacro MUI_HEADER_TEXT "$(setup_roles_title)" "$(setup_roles_subtitle)"
  nsDialogs::Create 1018
  Pop $0
  ${IfThen} $0 == error ${|} Abort ${|}

  ${NSD_CreateCheckbox} 0 8u 100% 12u "$(setup_role_server)"
  Pop $UN_ServerBox
  ${NSD_SetState} $UN_ServerBox $UN_Server
  ${NSD_OnClick} $UN_ServerBox usbnexus_RolesClicked

  ${NSD_CreateCheckbox} 0 30u 100% 12u "$(setup_role_client)"
  Pop $UN_ClientBox
  ${NSD_SetState} $UN_ClientBox $UN_Client
  ${NSD_OnClick} $UN_ClientBox usbnexus_RolesClicked

  ${NSD_CreateLabel} 12u 44u -12u 30u ""
  Pop $UN_Note

  ${NSD_CreateCheckbox} 0 82u 100% 12u "$(setup_role_web)"
  Pop $UN_WebBox
  ${NSD_SetState} $UN_WebBox $UN_Web
  ${NSD_OnClick} $UN_WebBox usbnexus_RolesClicked

  Call usbnexus_RolesUpdate
  nsDialogs::Show
FunctionEnd

Function usbnexus_RolesClicked
  Pop $0 ; the control
  Call usbnexus_RolesUpdate
FunctionEnd

; Shows the usbip-win2 note while "client" is ticked, and allows "Next"
; only with at least one of server and client.
Function usbnexus_RolesUpdate
  ${NSD_GetState} $UN_ServerBox $UN_Server
  ${NSD_GetState} $UN_ClientBox $UN_Client
  ${NSD_GetState} $UN_WebBox $UN_Web
  ${If} $UN_Client == ${BST_CHECKED}
    ${If} $UN_UsbipNeed == 1
      ${NSD_SetText} $UN_Note "$(setup_usbip_install_note)"
    ${ElseIf} $UN_UsbipNeed == 2
      ${NSD_SetText} $UN_Note "$(setup_usbip_update_note)"
    ${Else}
      ${NSD_SetText} $UN_Note "$(setup_usbip_present_note)"
    ${EndIf}
    ShowWindow $UN_Note ${SW_SHOW}
  ${Else}
    ShowWindow $UN_Note ${SW_HIDE}
  ${EndIf}
  GetDlgItem $0 $HWNDPARENT 1
  ${If} $UN_Server == ${BST_CHECKED}
  ${OrIf} $UN_Client == ${BST_CHECKED}
    EnableWindow $0 1
  ${Else}
    EnableWindow $0 0
  ${EndIf}
FunctionEnd

Function usbnexus_RolesLeave
  Call usbnexus_RolesUpdate
  StrCpy $UN_PagesShown 1
FunctionEnd

; ------------------------------------------------------------------ web page

Function usbnexus_WebPage
  ${IfThen} $PassiveMode = 1 ${|} Abort ${|}
  ${IfThen} $UN_Web != 1 ${|} Abort ${|}
  !insertmacro MUI_HEADER_TEXT "$(setup_web_title)" "$(setup_web_subtitle)"
  nsDialogs::Create 1018
  Pop $0
  ${IfThen} $0 == error ${|} Abort ${|}

  ${NSD_CreateLabel} 0 10u 70u 10u "$(setup_web_access)"
  Pop $0
  ${NSD_CreateFirstRadioButton} 74u 8u -74u 12u "$(setup_web_local)"
  Pop $UN_LocalRadio
  ${NSD_CreateAdditionalRadioButton} 74u 22u -74u 12u "$(setup_web_network)"
  Pop $UN_NetRadio
  ${If} $UN_WebLan == 1
    ${NSD_Check} $UN_NetRadio
  ${Else}
    ${NSD_Check} $UN_LocalRadio
  ${EndIf}

  ${NSD_CreateLabel} 0 46u 70u 10u "$(setup_web_port)"
  Pop $0
  ${NSD_CreateNumber} 74u 44u 40u 12u "$UN_WebPort"
  Pop $UN_PortBox
  ${NSD_OnChange} $UN_PortBox usbnexus_PortChanged
  ${NSD_CreateLabel} 120u 46u -120u 10u ""
  Pop $UN_PortStatus

  ${NSD_CreateLabel} 0 70u 70u 10u "$(setup_web_password)"
  Pop $0
  ${NSD_CreatePassword} 74u 68u 130u 12u ""
  Pop $UN_Pw1
  ${NSD_OnChange} $UN_Pw1 usbnexus_PasswordChanged
  ${NSD_CreateLabel} 0 88u 70u 10u "$(setup_web_password_repeat)"
  Pop $0
  ${NSD_CreatePassword} 74u 86u 130u 12u ""
  Pop $UN_Pw2
  ${NSD_OnChange} $UN_Pw2 usbnexus_PasswordChanged
  ${If} $UN_HasPw == 1
    ${NSD_CreateLabel} 74u 102u -74u 20u "$(setup_web_password_keep)"
  ${Else}
    ${NSD_CreateLabel} 74u 102u -74u 20u "$(setup_web_password_hint)"
  ${EndIf}
  Pop $0

  Call usbnexus_CheckPort
  Call usbnexus_WebUpdate
  nsDialogs::Show
FunctionEnd

Function usbnexus_PortChanged
  Pop $0 ; the control
  Call usbnexus_CheckPort
  Call usbnexus_WebUpdate
FunctionEnd

Function usbnexus_PasswordChanged
  Pop $0 ; the control
  Call usbnexus_WebUpdate
FunctionEnd

; Checks the entered port with the service executable (free, or already
; used by our own web interface).
Function usbnexus_CheckPort
  StrCpy $UN_PortOk 0
  ${NSD_GetText} $UN_PortBox $0
  StrLen $1 $0
  ${If} $1 < 1
  ${OrIf} $1 > 5
    ${NSD_SetText} $UN_PortStatus "$(setup_web_port_invalid)"
    Return
  ${EndIf}
  ${If} $0 < 1
  ${OrIf} $0 > 65535
    ${NSD_SetText} $UN_PortStatus "$(setup_web_port_invalid)"
    Return
  ${EndIf}
  nsExec::Exec '"$PLUGINSDIR\usbnexus.exe" web check-port $0'
  Pop $1
  ${If} $1 == 0
    StrCpy $UN_PortOk 1
    ${NSD_SetText} $UN_PortStatus "$(setup_web_port_free)"
  ${Else}
    ${NSD_SetText} $UN_PortStatus "$(setup_web_port_busy)"
  ${EndIf}
FunctionEnd

; "Next" needs a usable port and either a new password (at least 8
; characters, typed twice) or, on upgrades, both fields empty.
Function usbnexus_WebUpdate
  ${NSD_GetText} $UN_Pw1 $1
  ${NSD_GetText} $UN_Pw2 $2
  StrLen $3 $1
  StrLen $4 $2
  StrCpy $5 0
  ${If} $3 >= 8
  ${AndIf} $1 S== $2
    StrCpy $5 1
  ${ElseIf} $UN_HasPw == 1
  ${AndIf} $3 == 0
  ${AndIf} $4 == 0
    StrCpy $5 1
  ${EndIf}
  ${IfThen} $UN_PortOk != 1 ${|} StrCpy $5 0 ${|}
  GetDlgItem $0 $HWNDPARENT 1
  EnableWindow $0 $5
FunctionEnd

Function usbnexus_WebLeave
  ${NSD_GetState} $UN_NetRadio $UN_WebLan
  ${NSD_GetText} $UN_PortBox $UN_WebPort
  ${NSD_GetText} $UN_Pw1 $UN_WebPw
FunctionEnd

; ------------------------------------------------------------ after copying

; Installs and starts the service with the chosen roles and web settings,
; installs usbip-win2 for the client role and remembers the choices.
Function usbnexus_Configure
  Call usbnexus_LoadChoices
  StrCpy $1 '"$INSTDIR\usbnexus.exe" service install'
  ${IfThen} $UN_Server != 1 ${|} StrCpy $1 "$1 --no-server" ${|}
  ${IfThen} $UN_Client != 1 ${|} StrCpy $1 "$1 --no-client" ${|}
  ${If} $UN_PagesShown == 1
    ${If} $UN_Web == 1
      ${If} $UN_WebLan == 1
        StrCpy $1 "$1 --web network"
      ${Else}
        StrCpy $1 "$1 --web local"
      ${EndIf}
      StrCpy $1 "$1 --web-port $UN_WebPort"
      ${If} $UN_WebPw != ""
        ; Passed in a file (UTF-16 with BOM) so it is not on a command line.
        FileOpen $2 "$PLUGINSDIR\web-password" w
        FileWriteWord $2 0xFEFF
        FileWriteUTF16LE $2 $UN_WebPw
        FileClose $2
        StrCpy $1 '$1 --web-password-file "$PLUGINSDIR\web-password"'
      ${EndIf}
    ${Else}
      StrCpy $1 "$1 --web off"
    ${EndIf}
  ${EndIf}
  nsExec::ExecToLog $1
  Pop $0
  Delete "$PLUGINSDIR\web-password"
  ${If} $0 != 0
    MessageBox MB_OK|MB_ICONEXCLAMATION "$(setup_service_failed)" /SD IDOK
  ${EndIf}

  ${If} $UN_Client == 1
  ${AndIf} $UN_UsbipNeed != 0
    Call usbnexus_usbip_win2_install
  ${EndIf}

  WriteRegStr HKLM "${USBNEXUS_SETUP_KEY}" "Server" $UN_Server
  WriteRegStr HKLM "${USBNEXUS_SETUP_KEY}" "Client" $UN_Client
  ${If} $UN_PagesShown == 1
    WriteRegStr HKLM "${USBNEXUS_SETUP_KEY}" "Web" $UN_Web
    WriteRegStr HKLM "${USBNEXUS_SETUP_KEY}" "WebLan" $UN_WebLan
    WriteRegStr HKLM "${USBNEXUS_SETUP_KEY}" "WebPort" $UN_WebPort
  ${EndIf}
FunctionEnd

; Leaving the finish page: opens the web interface just set up, in the
; user's (not the elevated installer's) default browser. When Windows is
; about to restart (usbip-win2), it opens once after the next sign-in.
Function usbnexus_FinishLeave
  ${If} $UN_PagesShown == 1
  ${AndIf} $UN_Web == 1
    StrCpy $0 "https://localhost:$UN_WebPort/"
    ${If} ${RebootFlag}
      WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\RunOnce" "USB Nexus web" '"$WINDIR\explorer.exe" "$0"'
    ${Else}
      nsis_tauri_utils::RunAsUser "$WINDIR\explorer.exe" "$0"
    ${EndIf}
  ${EndIf}
FunctionEnd
