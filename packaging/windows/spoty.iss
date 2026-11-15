#ifndef Binary
  #error Binary must be defined on the ISCC command line
#endif
#ifndef OutputDir
  #error OutputDir must be defined on the ISCC command line
#endif

[Setup]
AppId={{DE4F4173-7AC1-4516-A460-F65189FBE53A}
AppName=Spoty
AppVersion=0.1.0
AppPublisher=Maks45-ui
AppSupportURL=https://github.com/Maks45-ui/Spoty/issues
DefaultDirName={localappdata}\Programs\Spoty
DefaultGroupName=Spoty
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
LicenseFile=..\..\LICENSE
OutputDir={#OutputDir}
OutputBaseFilename=Spoty-Setup-x64
SetupIconFile=spotsie.ico
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
UninstallDisplayIcon={app}\spoty.exe

[Files]
Source: "{#Binary}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\Spoty"; Filename: "{app}\spoty.exe"

[Run]
Filename: "{app}\spoty.exe"; Description: "Launch Spoty"; Flags: nowait postinstall skipifsilent
