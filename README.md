# prospect

A timeseries data analysis tool and (partial) visualizer.

Created by Ryan Lanigan (rlanigan@pdx.edu, ryanclanigan@gmail.com) for CS410: Rust Programming with Bart Massey.

# What is this?

Prospect is a very, very tiny re-implementation of a time-series analysis tool, inspired by and serving as an alternate language proof-of-concept of the program Seeq, by Seeq Corporation (I currently work there). To that end, it deals with timeseries data, which is time-ordered data where each data point has both a time and a value. `Prospect` refers to these as _samples_. When you have many samples ordered by their time value (or key), you have what `prospect` refers to as a _signal_. `Prospect` currently provides a single way to upload signals: csv files, which can only have two columns, the first of which is the time according to the ISO8601 standard and the second is the series of values. Every sample must have a value, but those values can be strings or floats (only one datatype per signal). On top of this, `prospect` provides a single operation that can be publicly made against signals: add, which adds two signals by key, creating a new resulting sample if only one signal has a given key. `Prospect` stores each signal uploaded and each signal created by a user-driven operation (currently only add) and can return these signals in one of two forms: csv and png. The csv file is the raw way this is stored by `prospect`, and the png is a facsimile of the kinds of visualizations the actual Seeq application makes.

An important thing to note is that nothing of the implementation of the concepts represented in `prospect` are taken from the real Seeq application. I implemented everything here from the ground up in my own way, in an attempt to see what a ground up Rust solution would look like (which is in fact quite different from what a Java solution looks like).

## How to build

Install the rust toolchain, download the repo, and run `cargo run` or `cargo run --release`. This will spawn a webserver at `localhost:3000` which will be listening for commands.

## Workflow

The current workflow of using `prospect` is as such:

1. Upload your signal(s) by doing a POST to `localhost:3000/signal` (The file must follow the above specification and also have a new line at the bottom)
2. Get the IDs of your currently uploaded signals by doing a GET to `localhost:3000/signal`
3. Add two signals together by doing a GET to `localhost:3000/operations/add` with the IDs specified in the JSON body.
4. Retrieve the signals you wish to see, either by using `localhost:3000/signal/{id}/png` or `localhost:3000/signal/{id}/csv`

There are two provided csv files that you can use to play with the system. If you would like to test add, upload either file twice, and then add the two resulting IDs together.

## Complete API detail

The API isn't complicated, but I couldn't find a good way to have actix auto generate API docs, so I'll write them out here.

### Operations API

**GET /operations/add** - Adds together the two provided signals and saves the resulting signal, allowing you to use it in further operations. The JSON input is in the below format.

```json
{
  "inputs": ["(ID's here separated by comma)"]
}
```

### Signal API

**GET /signal** - Returns a list of all signals currently saved by `prospect`

**POST /signal** - Posts a signal by csv to `prospect`, storing it and returning the ID of the newly created signal. The csv file should be attached as form data, should have a newline at the bottom, and be in the following form:

```
time, value
(Various datums here)
```

The time values should be a UTC ISO8601 timestamp of the form 2020-04-05T17:18:56Z. I specifically only tested to guarantee behavior on this kind of timestamp because time zones are hard and this simplifies the process. (I recommend using postman. It makes your life easier)

**GET /signal/{ID}/csv** - Returns the signal as a csv file, which is actually immediately uploadable to the server.

**GET /signal/{ID}/png** - Returns the signal as a png, which if done in a browser will conveniently display it. _NOTE_: String signals cannot be displayed as PNGs because no visualization library that I found current provided a way to visualize string datasets. You'll get a message telling you that if you try, but I figured I'd say that here too.

# Writeup

Everything below here is more of a writeup than a proper `README`, and was written for the reporting requirement for CS410: Rust Programming's final project.

## What was built

`Prospect` can be separated into a number of distinct components.

We have the primitives, which are the primitive types that timeseries calculations are computed against. These consist of `Signal`, `Sample`, `Scalar`, and `F64`, a home-grown float type because I need floats to both be `Ord` and `Eq`.

We then have the drivers, which can convert to/from some other form to a primitive. Currently, we have the csv driver, which allows parsing csv files to create signals, and the graph_generator driver, which can convert a signal to an equivalent png output. Drivers are fairly distinct from another, so they currently do not have a base trait of any sort. This will likely change over time.

Then come the operations, which are operations that can be applied against a specific set of primitives. I went back and forth on the design on the operations, and probably spent more time rewriting that code then actually writing any other part of the program. That being said, the current operations are `AddScalar`, `AddSample`, `AddSignal`, `BoundarySignal`, and `ExtremesSignal`. The only publically accessible signal (via the rest API) is `AddSignal`, though there's no reason I couldn't make boundary and extremes public as well. They currently only exist for convenience in determining the left-right (boundary) and top-bottom (extremes) bounds of the png visualization.

The server is up next, which itself consists of a small set of input and output classes, along with 2 sets of controllers for the endpoints. I use actix, which is _OK_, and not much more than that. I wouldn't recommend using it, but it was fairly simple to setup once I figured out its configuration system. The endpoints themselves are documented in `README.md`, so I'd recommend reading that for a good overview of the endpoints. `main.rs` ties into the server, and actually launches the server after some minor startup configuration.

There is also the `storage` module, which uses the csv driver to write and read signals from the internal file storage for the server. I don't technically need it, but its very convenient.

I wrote unit tests where appropriate, and I expect even after I write this I will go back and write some more because you can never have enough unit tests. That being said, unit tests are the only real tests I have. I have "real world" tests where I ran the project and verified it worked, but overall this project would benefit from a set of integration tests that really test the various endpoints. If I continue working on this (which I haven't decided on yet), I'll definitely add a bigger suite of tests. That being said, I'm _extremely_ confident in the functionality of the core operations (adding, boundary, extremes), and in the functionality of the core primitives. I'm also confident in the reader/writer because they use a well tested csv library. The only area I'm not particularly confident in is the API endpoints themselves, as they would need full integration testing to get working.

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

Rust is good. People should use it more. I mean I'm definitely going to use it more.
