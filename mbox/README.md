# mbox format parser

This crate implements a parser for the mbox email folder format as described
in [RFC4155 Appendix A](https://www.rfc-editor.org/rfc/rfc4155#appendix-A).

It is very early days for this code. Next steps:

1. Modify the API to being an iterator generating u8 slice references 
2. Detect and fail when a message is longer than the buffer
3. Gracefully handle long messages. I'm thinking allocate a growable
   buffer (with a limit) and copy data from the buffer into it until
   the end is found.
5. Figure out how to provide an iterator of messages
