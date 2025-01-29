[Setup]
AppName=HVCL_TC
AppVersion=1.0
DefaultDirName={pf}\HVCL_TC
DefaultGroupName=HVCL_TC
OutputDir=.
OutputBaseFilename=hvcl_tc_installer
Compression=lzma
SolidCompression=yes

[Files]
Source: "target\release\hvcl_tc.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\HVCL_TC"; Filename: "{app}\hvcl_tc.exe"
Name: "{group}\Uninstall HVCL_TC"; Filename: "{uninstallexe}"