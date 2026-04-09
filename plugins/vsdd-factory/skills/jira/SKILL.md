---
name: jira
description: Reference documentation for the ankitpokhrel jira-cli tool used for Jira integration in factory workflows. Reference-only skill, not directly invokable.
disable-model-invocation: true
---

# Jira CLI Reference

This project uses [ankitpokhrel/jira-cli](https://github.com/ankitpokhrel/jira-cli) for Jira integration.

## Jira Projects

| Project Key | Purpose | Contents |
|-------------|---------|----------|
| **DPGR** | Product Discovery | Initiatives tracking, ideation, product roadmap, discovery research |
| **DPGD** | Delivery | Epics, stories, tasks, subtasks — implementation work |

## Setup

```bash
export JIRA_API_TOKEN="your_api_token"
jira init
```

## Common Commands

### List & Search Issues

```bash
jira issue list                          # Recent issues
jira issue list -a$(jira me)             # Assigned to me
jira issue list -s"In Progress"          # By status
jira issue list -q "summary ~ keyword"   # JQL query
jira issue list --created -7d            # Last 7 days
jira issue list -yHigh -s"To Do" -lbackend  # Multiple filters
jira issue list --plain                  # Plain output (also: --raw, --csv)
```

### View Issue Details

```bash
jira issue view ISSUE-1
jira issue view ISSUE-1 --comments 5
```

### Create Issues

```bash
jira issue create                                          # Interactive
jira issue create -tTask -s"Summary" -b"Description" --no-input  # Direct
jira issue create -tStory -s"Story summary" -PEPIC-42      # Attach to epic
jira issue create --template /path/to/template.md          # From template
echo "Description" | jira issue create -s"Summary" -tTask  # From stdin
```

### Edit & Update Issues

```bash
jira issue edit ISSUE-1
jira issue assign ISSUE-1 "User Name"
jira issue assign ISSUE-1 $(jira me)    # Assign to self
jira issue assign ISSUE-1 x             # Unassign
```

### Transition Issues (Move Status)

```bash
jira issue move ISSUE-1 "In Progress"
jira issue move ISSUE-1 "In Progress" --comment "Started work"
jira issue move ISSUE-1 Done -RFixed -a$(jira me)
```

### Comments

```bash
jira issue comment add                                # Interactive
jira issue comment add ISSUE-1 "Comment text"         # Direct
jira issue comment add ISSUE-1 "Internal note" --internal
```

### Sprint Management

```bash
jira sprint list                                # Explorer view
jira sprint list --table                        # Table view
jira sprint list --current                      # Current sprint
jira sprint list --current -a$(jira me)         # My issues in current sprint
jira sprint list SPRINT_ID                      # Specific sprint
jira sprint list --state future,active          # Future and active
jira sprint add                                 # Interactive add
jira sprint add SPRINT_ID ISSUE-1 ISSUE-2       # Direct add (up to 50)
```

### Epic Management

```bash
jira epic add EPIC-KEY ISSUE-1 ISSUE-2
```

### Worklog / Time Tracking

```bash
jira issue worklog add                                          # Interactive
jira issue worklog add ISSUE-1 "2h 30m" --no-input             # Direct
jira issue worklog add ISSUE-1 "1h" --comment "Code review" --no-input
```

### Other Useful Commands

```bash
jira project list                                    # All projects
jira board list                                      # All boards
jira me                                              # Current user
jira issue clone ISSUE-1                             # Clone
jira issue delete ISSUE-1                            # Delete
jira issue delete ISSUE-1 --cascade                  # Delete with subtasks
jira issue link remote ISSUE-1 https://example.com "Link text"
```

## Pagination

Results limited to **100 per request** by default.

```bash
jira issue list --paginate 20           # First 20
jira issue list --paginate 50:100       # 100 results from offset 50
```

### Counting Large Result Sets

```bash
count=0; offset=0
while true; do
  batch=$(jira sprint list SPRINT_ID --paginate ${offset}:100 --plain --no-headers 2>/dev/null | wc -l | tr -d ' ')
  count=$((count + batch))
  [ "$batch" -lt 100 ] && break
  offset=$((offset + 100))
done
echo "Total: $count"
```

## Tips

- Use `$(jira me)` to reference yourself in commands
- Most commands support `--no-input` to skip interactive prompts
- Use `-q` flag for raw JQL queries within project context
- Time formats: `1h`, `30m`, `2d 3h 30m`
- Default pagination is `0:100` — use `--paginate` for larger result sets
