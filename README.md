# rust-rocket-web-server-rewrite

This is a rewrite from scratch of the rust-rocket-web-server functionality, in order to create it from memory and without looking up any of the reference material from the video lesson that was used to learn and make the original.

A simple front end might also potentially be added to this version as a way of extending it.



The only material looked at besides memory is the Rocket documentation itself in order to set up the initial boilerplate, and then if I got stuck at any point I would check the original material while noting which parts those were.


List of things I had to look up in the source material due to getting stuck when using only memory recall:

1)  I had to look at the original code for this one line, as it wasn't working from writing it like this from memory
    // let results: rusqlite::Result<Vec<ToDoItem>> = rows.collect();

    For lines 74..87, I had to make brief glances at the source material to refresh myself on how the results were being collected and turned into the final output.


2)  Once the fetch_all_todo_items endpoint was completed, trying to access it showed that there was an error in the SQL statement portion of the code, so I reviewed the source material to check for errors in the SQL statements.  I saw that I was missing details like 'primary key' and 'not null' in the creation statements, but correcting these didn't solve the problem yet.

    I modified my error message to include the SQL error message from the Result instead of throwing it away, and it said there was no such table as todo_items

    I traced the problem back by reading my a println! of the result of creating the table, until I saw that it was failing to create it because it said "create table it not exists" instead of "create table if not exists"



3)  Mention of things I had to ask github copilot to remind me of:

    I knew that in cargo.toml, I had to add a feature to rusqlite to make it work properly, but forgot what it was called.  It provided me with a reminder:
        - rusqlite = { version = "0.32.1", features = ["bundled"] }


Besides these things, I was able to produce the functionality of the original Rocker server example without having to look at anything to remind myself of how.

Now that this is done, the next step will be to investigate ways to add a front end to it so that the todo list can be fetched and rendered as something like an unordered list, with each item having a delete button to remove it, and a button to add new items to the list.

