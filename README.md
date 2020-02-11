# prospect

A timeseries data analysis tool and (partial) visualizer.

# What is this?

Prospect is a very, very tiny re-implementation of a time-series analysis tool, inspired by and serving as an alternate language proof-of-concept of the program Seeq, by Seeq Corporation (I currently work there). To that end, it deals with timeseries data, which is time-ordered data where each data point has both a time and a value. `Prospect` refers to these as _samples_. When you have many samples ordered by their time value (or key), you have what `prospect` refers to as a _signal_. `Prospect` currently provides a single way to upload signals: csv files, which can only have two columns, the first of which is the time according to the ISO8601 standard and the second is the series of values. Every sample must have a value, but those values can be strings or floats (only one datatype per signal). On top of this, `prospect` provides a single operation that can be publicly made against signals: add, which adds two signals by key, creating a new resulting sample if only one signal has a given key. `Prospect` stores each signal uploaded and each signal created by a user-driven operation (currently only add) and can return these signals in one of two forms: csv and png. The csv file is the raw way this is stored by `prospect`, and the png is a facsimile of the kinds of visualizations the actual Seeq application makes.

## How to build

Install the rust toolchain, download the repo, and run `cargo run` or `cargo run --release`. This will spawn a webserver at `localhost:3000` which will be listening for commands.

## Workflow

The current workflow of using `prospect` is as such:

1. Upload your signal(s) by doing a POST to `localhost:3000/signal` (The file must follow the above specification and also have a new line at the bottom)
2. Get the IDs of your currently uploaded signals by doing a GET to `localhost:3000/signal`
3. Add two signals together by doing a GET to `localhost:3000/operations/add` with the IDs specified in the JSON body.
4. Retrieve the signals you wish to see, either by using `localhost:3000/signal/{id}/png` or `localhost:3000/signal/{id}/csv`

There are two provided csv files that you can use to play with the system. If you would like to test add, upload either file twice, and then add the two resulting IDs together

## Complete API detail

The API isn't complicated, but I couldn't find a good way to have actix auto generate API docs, so I'll write them out here.

### Operations API

**GET /operations/add** - Adds together the two provided signals and saves the resulting signal, allowing you to use it in further operations. The JSON input is in the below format.

```json
{
    inputs: [
        (ID's here separated by comma)
    ]
}
```

### Signal API

**GET /signal** - Returns a list of all signals currently saved by `prospect`

**POST /signal** - Posts a signal by csv to `prospect`, storing it and returning the ID of the newly created signal. The csv file should be attached as form data, should have a newline at the bottom, and be in the following form:

```
time, value
(Various datums here)
```

(I recommend using postman. It makes your life easier)

**GET /signal/{ID}/csv** - Returns the signal as a csv file, which is actually immediately uploadable to the server.

**GET /signal/{ID}/png** - Returns the signal as a png, which if done in a browser will conveniently display it. _NOTE_: String signals cannot be displayed as PNGs because no visualization library that I found current provided a way to visualize string datasets. You'll get a message telling you that if you try, but I figured I'd say that here too.
