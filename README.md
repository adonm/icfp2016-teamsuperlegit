# Developing an origami unfolder in Rust

Each year the [International Conference on Functional Programming](http://www.icfpconference.org) holds a ["a fun and challenging three-day programming competition"](http://www.icfpconference.org/contest) - this repository is our effort and some thoughts along the way.

## Team Super Legit

First up our (super legit) team consisted of Rowan, Scott, Patrick, Matt, Adon. A few of us had dabbled in Rust prior to the contest, but for the most part there was a couple hours of pre-reading before  engaging full combat with the borrow checker (and begging pointers/references/structs to stop hiding in closures!).

If you are getting into Rust, the below references were top shelf:
 * [The Rust Book](https://doc.rust-lang.org/book/)
 * [New Rustacean Podcast](http://www.newrustacean.com/)
 * [Rustlings](https://github.com/carols10cents/rustlings) - great for befriending the compiler

![super legit team](superlegitteam.gif?raw=true "Team Super Legit")

## Day 1 - Problem announced
Ohhh it's like a bit of trig with some tricky looking fractions... We can (with v.little actual Rust) download and commit problems, and draw a square as an SVG (our visualiser was inkscape, and our API library was curl, coz we are but unix men). This was also the day we discovered that rational numbers were quite new to Rust, and as such supporting libraries for matrix and geometry operations were lacking.

## Day 2 - Such Trig Much Wow
OK so now Rowan had nutted out base structures for generic number types (for floats/bigrationals), matrices, andmatrix operations, Patrick out slicey-edges to cut some paper, and Matt gets close and personal with what folding really is. Scott figures out how to stitch polygons together for figuring out our own semblance scores (in the end we just abused the API and submitted everything). Adon sorted out how to anchor the unit square of origami paper to the largest corner in the folded silhouette, and drew some stuff as SVGs. We all gave up on the idea of submitting problems for other teams to solve as there was too much left to be done on our frankensteins solver to bring it to life!

## Day 3 - A New Hope
Geometric operations are go! Now we just had to figure out rational rotations/distance calcs. Oh wait. They both require roots. We are rooted. Ah but we can pretend that roots are rationals with the magic of brute force, and even some quantizing magic. Wow some stuffs getting solved. Adon doesn't know his x's from y's (he's been reading to much genderfluid fanfic). Rowan figures out how to plug all the holes caused by relentless float conversions. Patrick develops multifold (we can fold an origami more than once!) magics and scripts a bailout to scattershot solve anything and submit everything while staying under API limits. TBH our code was super ~legit~ quick coz we only brute forced the tiniest bits at the end, but it also only solved a tiny fraction of the problems posted.

## Conclusion

Most of the team appeared to enjoy themselves, though working from Perth, Australia with a foreigner from Shoreditch, London was interesting. He kept making us use his companies fancy videoconferencing software (OK it had a pretty cool whiteboard) to 'synergise' and 'figure out wtf everyone is up to' which turned out great. Poking around with task management in Trello was ok, though in this timeframe/pressure it was easier to just yell at eachother, and many more tasks were created than completed. It was tricky trying to make a generic number type in Rust to handle floats and BigRationals, but we had basically ported everything but a couple steps requiring square roots back to big rationals by the end.
