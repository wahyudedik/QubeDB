; QubeDB Windows Installer Script
; NSIS script untuk membuat installer executable

!define PRODUCT_NAME "QubeDB"
!define PRODUCT_VERSION "1.0.0"
!define PRODUCT_PUBLISHER "QubeDB Team"
!define PRODUCT_WEB_SITE "https://qubedb.com"
!define PRODUCT_DIR_REGKEY "Software\Microsoft\Windows\CurrentVersion\App Paths\qubedb.exe"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
!define PRODUCT_UNINST_ROOT_KEY "HKLM"

; MUI Settings
!include "MUI2.nsh"
!define MUI_ABORTWARNING
!define MUI_ICON "qubedb.ico"
!define MUI_UNICON "qubedb.ico"

; Welcome page
!insertmacro MUI_PAGE_WELCOME

; License page
!insertmacro MUI_PAGE_LICENSE "LICENSE.txt"

; Components page
!insertmacro MUI_PAGE_COMPONENTS

; Directory page
!insertmacro MUI_PAGE_DIRECTORY

; Instfiles page
!insertmacro MUI_PAGE_INSTFILES

; Finish page
!define MUI_FINISHPAGE_RUN "$INSTDIR\qubedb-gui.exe"
!define MUI_FINISHPAGE_RUN_TEXT "Launch QubeDB Desktop"
!define MUI_FINISHPAGE_SHOWREADME "$INSTDIR\README.txt"
!insertmacro MUI_PAGE_FINISH

; Uninstaller pages
!insertmacro MUI_UNPAGE_INSTFILES

; Language files
!insertmacro MUI_LANGUAGE "English"

; MUI Settings
Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"
OutFile "QubeDB-Setup.exe"
InstallDir "$PROGRAMFILES\QubeDB"
InstallDirRegKey HKLM "${PRODUCT_DIR_REGKEY}" ""
ShowInstDetails show
ShowUnInstDetails show

; Version Information
VIProductVersion "${PRODUCT_VERSION}.0"
VIAddVersionKey "ProductName" "${PRODUCT_NAME}"
VIAddVersionKey "ProductVersion" "${PRODUCT_VERSION}"
VIAddVersionKey "CompanyName" "${PRODUCT_PUBLISHER}"
VIAddVersionKey "LegalCopyright" "© 2025 QubeDB Team"
VIAddVersionKey "FileDescription" "QubeDB Multi-Model Database"
VIAddVersionKey "FileVersion" "${PRODUCT_VERSION}"

Section "QubeDB Core" SEC01
  SetOutPath "$INSTDIR"
  SetOverwrite ifnewer
  
  ; Core binaries
  File "qubedb.exe"
  File "qubedb-gui.exe"
  
  ; GUI assets
  File /r "gui\*"
  
  ; Documentation
  File "README.txt"
  File "LICENSE.txt"
  File "CHANGELOG.txt"
  
  ; Create data directory
  CreateDirectory "$INSTDIR\data"
  CreateDirectory "$INSTDIR\logs"
  CreateDirectory "$INSTDIR\config"
  
  ; Create default config
  FileOpen $0 "$INSTDIR\config\qubedb.conf" w
  FileWrite $0 "# QubeDB Configuration File$\r$\n"
  FileWrite $0 "host = localhost$\r$\n"
  FileWrite $0 "port = 8080$\r$\n"
  FileWrite $0 "data_dir = $INSTDIR\data$\r$\n"
  FileWrite $0 "log_level = info$\r$\n"
  FileWrite $0 "max_connections = 100$\r$\n"
  FileClose $0
SectionEnd

Section "Desktop Shortcuts" SEC02
  CreateDirectory "$SMPROGRAMS\QubeDB"
  CreateShortCut "$SMPROGRAMS\QubeDB\QubeDB Desktop.lnk" "$INSTDIR\qubedb-gui.exe"
  CreateShortCut "$SMPROGRAMS\QubeDB\QubeDB Command Line.lnk" "$INSTDIR\qubedb.exe"
  CreateShortCut "$SMPROGRAMS\QubeDB\Documentation.lnk" "$INSTDIR\README.txt"
  CreateShortCut "$SMPROGRAMS\QubeDB\Uninstall.lnk" "$INSTDIR\uninst.exe"
  
  ; Desktop shortcuts
  CreateShortCut "$DESKTOP\QubeDB Desktop.lnk" "$INSTDIR\qubedb-gui.exe"
SectionEnd

Section "Windows Service" SEC03
  ; Install as Windows service
  ExecWait '"$INSTDIR\qubedb.exe" --install-service'
SectionEnd

Section "Environment Variables" SEC04
  ; Add to PATH
  EnVar::SetHKLM
  EnVar::AddValue "PATH" "$INSTDIR"
  
  ; Set QUBEDB_HOME
  WriteRegStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "QUBEDB_HOME" "$INSTDIR"
SectionEnd

; Section descriptions
!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC01} "Core QubeDB database engine and GUI application"
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC02} "Create desktop and start menu shortcuts"
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC03} "Install QubeDB as a Windows service (automatic startup)"
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC04} "Add QubeDB to system PATH and set environment variables"
!insertmacro MUI_FUNCTION_DESCRIPTION_END

Section -AdditionalIcons
  WriteIniStr "$INSTDIR\${PRODUCT_NAME}.url" "InternetShortcut" "URL" "${PRODUCT_WEB_SITE}"
  CreateShortCut "$SMPROGRAMS\QubeDB\Website.lnk" "$INSTDIR\${PRODUCT_NAME}.url"
  CreateShortCut "$SMPROGRAMS\QubeDB\Uninstall.lnk" "$INSTDIR\uninst.exe"
SectionEnd

Section -Post
  WriteUninstaller "$INSTDIR\uninst.exe"
  WriteRegStr HKLM "${PRODUCT_DIR_REGKEY}" "" "$INSTDIR\qubedb.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayName" "$(^Name)"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "UninstallString" "$INSTDIR\uninst.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayIcon" "$INSTDIR\qubedb.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "URLInfoAbout" "${PRODUCT_WEB_SITE}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "Publisher" "${PRODUCT_PUBLISHER}"
SectionEnd

Function un.onUninstSuccess
  HideWindow
  MessageBox MB_ICONINFORMATION|MB_OK "$(^Name) was successfully removed from your computer."
FunctionEnd

Function un.onInit
  MessageBox MB_ICONQUESTION|MB_YESNO|MB_DEFBUTTON2 "Are you sure you want to completely remove $(^Name) and all of its components?" IDYES +2
  Abort
FunctionEnd

Section Uninstall
  ; Stop and remove service
  ExecWait '"$INSTDIR\qubedb.exe" --uninstall-service'
  
  ; Remove files
  Delete "$INSTDIR\${PRODUCT_NAME}.url"
  Delete "$INSTDIR\uninst.exe"
  Delete "$INSTDIR\qubedb.exe"
  Delete "$INSTDIR\qubedb-gui.exe"
  Delete "$INSTDIR\README.txt"
  Delete "$INSTDIR\LICENSE.txt"
  Delete "$INSTDIR\CHANGELOG.txt"
  Delete "$INSTDIR\config\qubedb.conf"
  
  ; Remove directories
  RMDir "$INSTDIR\config"
  RMDir "$INSTDIR\gui"
  RMDir "$INSTDIR\data"
  RMDir "$INSTDIR\logs"
  RMDir "$INSTDIR"
  
  ; Remove shortcuts
  Delete "$SMPROGRAMS\QubeDB\*"
  RMDir "$SMPROGRAMS\QubeDB"
  Delete "$DESKTOP\QubeDB Desktop.lnk"
  
  ; Remove from PATH
  EnVar::SetHKLM
  EnVar::DeleteValue "PATH" "$INSTDIR"
  
  ; Remove environment variable
  DeleteRegValue HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "QUBEDB_HOME"
  
  ; Remove registry entries
  DeleteRegKey ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}"
  DeleteRegKey HKLM "${PRODUCT_DIR_REGKEY}"
  
  SetAutoClose true
SectionEnd
