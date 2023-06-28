# Connect to the debugger
target remote :2331

# Prevent panic corruption of LR
set backtrace limit 32

# Program code to flash
load

# Reset the device
monitor reset

# Set a breakpoint at main()
break main

# Run until we hit main()
continue