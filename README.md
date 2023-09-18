
no blanket implementations yet




// synthesis ideas:
// tanh tube amp clipping
// doom shit: filter, subtract one to get both, then fuck up each one separately, then re add
// possibly delay separately as well
// hardstyle kick
// FM synth
// additive synth
// that IIR shit / simulation shit
// ringmod example
// yea it would be pretty god damn easy to do arbitrary synthesis techniques here. like fm. 
// frequency sweep
// crossfade (incl. crossfade with self mmm)
// FIR impulses




// We are at the point where a lot of this shit is not strictly necessary for minimix but it is how you can generate sounds for minimix

// generate a really nice readme demonstrating eg each thing in signal_synth with plots of at least time domain shit
// its kinda jupyter notebook-esque
// gen_readme test

// A note, i think this still makes sense even as a layer in custom synth, needs to buffer, is maybe more latency, shrug
// like you would only swap the sound at discrete intervals of them modifying the params
// modulation interpolates between buffers?

// speculative note: idk resample, decimate, heterodyne?

// A 1d signal: also a ndvec. does it make sense to consider proj, dot product, etc?
// more shit it could have:
// * to_complex
// * hilbert
// * fft
// * ifft
// * trunc (take the transient off a snare or hat or something)
// * envelope follower (or get off hilbert)
// * plot time, plot freq


odo making of README generator / plottings, parameter exploration

todo

be good to guarantee that load sound play sound, sound is played

maybe a monadic operations sequence is better with always moving the vec
or like builder pattern.......


maybe sample rate agnostic and rad per sample is argument. is there a notation for rad/samp? or like period in samples
forwarding of len
synchronous play fn good

wat about do by strname js way.

white should probs be -1 to 1

what about doing wavetable shit of modulated sampling position
may better results to oversample...

signal.plt
fft.plt bonus
conv_fast no work

Sig
defs wanting that refactor toward monadic, move it, etc

plotting of some cool things and cool fft's of things
md of understanding ring mod