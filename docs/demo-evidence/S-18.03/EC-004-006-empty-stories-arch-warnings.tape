# EC-004 / EC-006 — Empty stories list and empty arch_files operator warnings
# EC-004: stories: [] triggers WARNING, injects arch_files + state_pointer only, exit 0
# EC-006: arch_files: [] triggers WARNING, injects story spec_files + state_pointer only, exit 0

Output docs/demo-evidence/S-18.03/EC-004-006-empty-stories-arch-warnings.gif
Output docs/demo-evidence/S-18.03/EC-004-006-empty-stories-arch-warnings.webm

Require bash

Set Theme "Dracula"
Set Width 1200
Set Height 700
Set Padding 20
Set FontSize 14

Type "# EC-004: stories: [] — WARNING + inject arch_files + state_pointer, exit 0"
Enter
Sleep 500ms
Type "bash /tmp/demo-ec004.sh"
Enter
Sleep 5s

Type ""
Enter
Sleep 400ms

Type "# EC-006: arch_files: [] — WARNING + inject story spec_files + state_pointer, exit 0"
Enter
Sleep 500ms
Type "bash /tmp/demo-ec006.sh"
Enter
Sleep 5s
