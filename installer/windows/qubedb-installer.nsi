; QubeDB Windows Installer
; NSIS script for QubeDB Desktop Installer

; --- General Setup ---
Name "QubeDB Desktop"
OutFile "QubeDB-Setup.exe"
InstallDir "$PROGRAMFILES\QubeDB Desktop"
RequestExecutionLevel admin

; --- Pages ---
Page directory
Page instfiles

; --- Uninstaller ---
UninstallText "This will uninstall QubeDB Desktop. Continue?"
UninstallIcon "${NSISDIR}\Contrib\Graphics\Icons\modern-uninstall.ico"

Section "Install"
    SetOutPath "$INSTDIR"

    ; Copy main application files
    File /r "..\..\qubedb-gui\target\release\qubedb-gui.exe"
    File /r "..\..\qubedb-core\target\release\qubedb-core.exe"
    File /r "..\..\qubedb-gui\dist\"
    File /r "..\..\qubedb-gui\icons\"

    ; Create Desktop Shortcut
    CreateShortcut "$DESKTOP\QubeDB Desktop.lnk" "$INSTDIR\qubedb-gui.exe" "" "$INSTDIR\icons\icon.ico" 0 "" "" "QubeDB Desktop"

    ; Create Start Menu Shortcut
    CreateDirectory "$SMPROGRAMS\QubeDB Desktop"
    CreateShortcut "$SMPROGRAMS\QubeDB Desktop\QubeDB Desktop.lnk" "$INSTDIR\qubedb-gui.exe" "" "$INSTDIR\icons\icon.ico" 0 "" "" "QubeDB Desktop"

    ; Add to PATH (for qubedb-core CLI)
    ; This is more complex with NSIS for system-wide PATH.
    ; For simplicity, we'll add it to the user's PATH or rely on direct execution.
    ; For system-wide, you'd need to modify HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment
    ; For now, we'll skip system PATH modification in NSIS for brevity.

    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QubeDB Desktop" "DisplayName" "QubeDB Desktop"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QubeDB Desktop" "UninstallString" "cmd.exe /C `"$INSTDIR\uninstall.exe`""
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QubeDB Desktop" "InstallLocation" "$INSTDIR"
    WriteUninstaller "uninstall.exe"
SectionEnd

Section "Uninstall"
    ; Remove Desktop Shortcut
    Delete "$DESKTOP\QubeDB Desktop.lnk"

    ; Remove Start Menu Shortcuts
    Delete "$SMPROGRAMS\QubeDB Desktop\QubeDB Desktop.lnk"
    RMDir "$SMPROGRAMS\QubeDB Desktop"

    ; Remove application files
    Delete /REBOOTOK "$INSTDIR\qubedb-gui.exe"
    Delete /REBOOTOK "$INSTDIR\qubedb-core.exe"
    RMDir /r /REBOOTOK "$INSTDIR\dist"
    RMDir /r /REBOOTOK "$INSTDIR\icons"
    Delete "$INSTDIR\uninstall.exe"

    ; Remove installation directory if empty
    RMDir "$INSTDIR"

    ; Remove registry entries
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\QubeDB Desktop"
SectionEnd