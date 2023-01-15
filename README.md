# maeve - a junk email filter solution

Refusing to give up running my own email server, and having used
the same email address for a very long time means that I get a
lot of junk email.

This is my email filtering testbed. The eventual goal is to have
something that can be used to filter emails without much human 
intervention.

## Ideas

As this project is so far in it's very early days, these are some ideas.

I would like to be able to implement a set of checks that are the
functional equivalents of the most high impact SpamAssassin rules, and 
run them on already received emails, comparing the spam scoring to
what SpamAssassin thinks.

* Find out a way to traverse mbox format files to be able to do analysis
  after emails have been received.
* Find out a way to detect already seen emails in an inbox
