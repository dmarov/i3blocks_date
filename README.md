plugin for date display on i3bar panel

add this to config
`
[datetime]
command=i3blocks_date "%d/%m %H:%M" $((3600 *3))
format=json
interval=1
color=#FFFFFF
`
