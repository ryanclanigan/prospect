# Ryan Lanigan CS410: Rust Programming Project Write-up

## What was built

`Prospect` can be separated into a number of distinct components.

We have the primitives, which are the primitive types that timeseries calculations are computed against. These consist of `Signal`, `Sample`, `Scalar`, and `F64`, a home-grown float type because I need floats to both be `Ord` and `Eq`.

We then have the drivers, which can convert to/from some other form to a primitive. Currently, we have the csv driver, which allows parsing csv files to create signals, and the graph_generator driver, which can convert a signal to an equivalent png output. Drivers are fairly distinct from another, so they currently do not have a base trait of any sort. This will likely change over time.

Then come the operations, which are operations that can be applied against a specific set of primitives. I went back and forth on the design on the operations, and probably spent more time rewriting that code then actually writing any other part of the program. That being said, the current operations are `AddScalar`, `AddSample`, `AddSignal`, `BoundarySignal`, and `ExtremesSignal`. The only publically accessible signal (via the rest API) is `AddSignal`, though there's no reason I couldn't make boundary and extremes public as well. They currently only exist for convenience in determining the left-right (boundary) and top-bottom (extremes) bounds of the png visualization.

The server is up next, which itself consists of a small set of input and output classes, along with 2 sets of controllers for the endpoints. I use actix, which is _OK_, and not much more than that. I wouldn't recommend using it, but it was fairly simple to setup once I figured out its configuration system. The endpoints themselves are documented in `README.md`, so I'd recommend reading that for a good overview of the endpoints. `main.rs` ties into the server, and actually launches the server after some minor startup configuration.

There is also the `storage` module, which uses the csv driver to write and read signals from the internal file storage for the server. I don't technically need it, but its very convenient.

I wrote unit tests where appropriate, and I expect even after I write this I will go back and write some more because you can never have enough unit tests.

## Related work

There really isn't much similar to this out in the world. There is `timeseries`, which is aiming to be a timeseries library that you can consume (https://github.com/klangner/timeseries.rs) but I'd argue that I actually have a somewhat more comprehensive timeseries abstraction. According to crate.io and google, there are about 3 timeseries databases, but `prospect` does operations against that data, which is fundamentally different from storing it.

Overall there is nothing I could really pull from, and even if I did pull from `timeseries`, my scope is so much larger than what is in that minimal library that it is not usable for `prospect`.

## What works well and what doesn't work well

Everything works mostly well. The only issue I have (which really bothers me) is that uploaded csv files need to have an empty line at the bottom. Other than that, everything works reasonably quickly, at least with small sets of data. Given that I am currently storing signals in csv files, it can't scale to well. That being said, importing a signal with 100,000 samples only took ~3 seconds, fetching that signal in csv form takes ~2 seconds, and fetching it as a png takes ~10 seconds. That's really not that bad, but I'm betting Ubuntu is caching some of these files to make my life easier. Even adding that signal to itself only took about 5 seconds. That's not terribly efficient (and much slower than my actual job's application) but honestly I'm pretty happy with the results. This is a usable application for minimal timeseries analysis, and with some proper refactoring, there might be some interesting applications for something like this.

There is one notable issue I'd like to call out before moving on: Rust's lack of reflection is annoying. In actual Seeq application that I work with regularly, our set of operations are dynamically loaded at runtime, because we have several hundred and its unreasonable to have to manually register each one. For our purposes, this is great. For the purposes of `prospect`, this _would_ be great. However, since Rust has no real concept of reflection due to its lack of a runtime, I either have to manually register each operation with some central list/map of all operations, or have each operation be its own endpoint, as I did for the add operation. Both of these are not particularly good solutions, but the central map is definitely more appealing. Realistically, if I move forward with this application, I will likely move the operators into a central map, but I won't be happy about it.

One more notable issue that I caused directly is that I basically threw errors to the wind in this. Realistically, I should have defined a custom enum to represent the different kinds of errors I could encounter, and then use those where appropriate. However, I just wrapped everything using a third-party error handling crate and used string-based errors whenever I needed to generate an error message. This is **bad** and I should probably feel bad about it, but it let me propagate error message surprisingly effectively and get to writing the rest of the code quickly.

## Lessons learned

In addition to general practice with Rust, I learned three identifiable lessons during the course of this project.

1. Rust is fantastic when you want to handle cases where data can be one of two variants (an enum). I use the `Scalar` enum all over in this code base, and the ability to have deterministic code set up at compile time for all different variations of an enum that each can contain different types of data is so incredibly powerful. I went back and forth during this project due to other classes and work using Javascript, PHP, Java, and of course Rust, and I missed Rust's extensive pattern matching on enums literally everyday I wasn't using it. There's nothing like it and its beautiful.

2. Rust's struct system is somewhat limiting, but I think that's because of how I think about programming. I inherently think of programming from a classical object oriented way via interfaces, abstract classes, inherited classes, and so on. Rust was difficult to adapt to because of how fundamentally different its struct + trait system is to my way of thinking. I found it difficult to design some of the more complicated parts of the system because I couldn't develop simple inheritance patterns to propagate methods and fields ac cross a variety of structs. For example, I wanted each operation to have more of a standardized structure, so I could just call something like `getArgs` on any arbitrary operation and get those arguments, but I didn't find a way to do that that didn't also result in a lot fairly annoying boilerplate. I think I might have been missing something, but I feel like Rust's structs are limiting out of unfamiliarity.

3. This is the most important lesson: Rust will be my language of choice for anything that I can spend some time writing. This language is just beautiful and I feel like my work in other languages directly benefits from the expressive power that Rust gives me. I am starting to think about the problems with just throwing exceptions (though they have their uses) and the power of being able to control what memory is allocated inherently. Python and Javascript will still be great for quicker projects that I just need to blast through or frontend specific work, but if I have the time and the opportunity, Rust is where its going to be.

# Conclusion

Rust is good. People should use it more.
