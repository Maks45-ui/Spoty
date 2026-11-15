#ifndef Binary
  #error Binary must be defined on the ISCC command line
#endif
#ifndef OutputDir
  #error OutputDir must be defined on the ISCC command line
#endif

[Setup]
AppId={{DE4F4173-7AC1-4516-A460-F65189FBE53A}
AppName=Spoty
AppVersion=0.2.2
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
SetupIconFile=spotifast.ico
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

[Registry]
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "Spoty"; ValueData: "{code:StartupCommand}"; Check: InitializeStartup; Flags: createvalueifdoesntexist
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: none; ValueName: "Spoty"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run"; ValueType: none; ValueName: "Spoty"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Maks45-ui\Spoty"; ValueType: dword; ValueName: "StartupInitialized"; ValueData: "1"; Flags: uninsdeletevalue uninsdeletekeyifempty

[Run]
Filename: "{app}\spoty.exe"; Description: "Launch Spoty"; Flags: nowait postinstall skipifsilent

[Code]
const
  RunKey = 'Software\Microsoft\Windows\CurrentVersion\Run';
  StartupApprovedKey = 'Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run';
  InstallKey = 'Software\Maks45-ui\Spoty';
  LegacyName = 'Spotifast';
  HiddenArgument = ' --start-hidden';

function ReadLegacyStartup(var Command: String): Boolean;
var
  Exe: String;
begin
  Result := False;
  if not RegQueryStringValue(HKCU, RunKey, LegacyName, Command) then
    Exit;
  Exe := ExpandConstant('{app}\spoty.exe');
  Result := (CompareText(Command, Exe) = 0) or
    (CompareText(Command, '"' + Exe + '"') = 0) or
    (CompareText(Command, Exe + HiddenArgument) = 0) or
    (CompareText(Command, '"' + Exe + '"' + HiddenArgument) = 0);
end;

function InitializeStartup: Boolean;
var
  Command: String;
  Approval: AnsiString;
begin
  Result := not RegValueExists(HKCU, InstallKey, 'StartupInitialized');
  if Result and ReadLegacyStartup(Command) then
    if RegQueryBinaryValue(HKCU, StartupApprovedKey, LegacyName, Approval) then
      if Length(Approval) > 0 then
        Result := (Ord(Approval[1]) and 1) = 0;
end;

function StartupCommand(Param: String): String;
begin
  if not ReadLegacyStartup(Result) then
    Result := '"' + ExpandConstant('{app}\spoty.exe') + '"' + HiddenArgument;
end;

procedure CurStepChanged(CurStep: TSetupStep);
var
  LegacyCommand: String;
begin
  if (CurStep = ssPostInstall) and ReadLegacyStartup(LegacyCommand) then
  begin
    RegDeleteValue(HKCU, RunKey, LegacyName);
    RegDeleteValue(HKCU, StartupApprovedKey, LegacyName);
  end;
end;
