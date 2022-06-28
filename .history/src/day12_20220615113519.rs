//In rust errors are grouped into two; the recoverable and irrecoverable errors.
//Recoverable errors are errors that can be related as a result of absence of files, values. The option<T> enum is used when an error is a recoverable error.
//Unrecoverable errors are errors that can be related to accessing a location of an array that has left it's scope, attempting to access and index that's not present. 
//The panic! macro is used for an unrecoverable error and stops the program.
//
