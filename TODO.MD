# Approach:

Initial approach changed, new approach: try open file with rust pass it base 64 to js code ✅

Now: implement what is in useWindowListeners
- next      (current)
- previous
- first
- last
- save
- new
- open

### CURRENT ISSUES:

- need to find a way to dynamically update list of media in backend
- need to handle file already named like the file being saved
- need to fix double call

# To Add

- Message when trying to do something else when one action is currently being executed
- Display a message when an action finished executing (ex: copied file ...)
- 


- Can we display name of file being viewed in the window?


# Ideas for the future

- Current path
- All paths (list)
- All medias

Commands:
- Randomize
- Sort
- 


Menu:
- Pick file type to display: gif, png, jpeg etc
- 


Command + F --> Search
Command + R --> Reverse
Command + P --> Print
Command + L --> Panel to visualize/scroll
Command + G --> Global
Command + something --> Local
Command + I --> Display informations

Other:
- Replace Vector of types by an enum
- instead of returning base64 data, return the whole gif with metadata, name & title etc;



selected paths: multiple: can return multiple paths?
