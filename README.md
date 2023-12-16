
# Dynamics Kinematics Environment in Rust [rDKE]

:construction: [Under Construction] :construction:

This is the dynamics kinematics simulation environment implemented in Rust. It 
will be a multi-purpose platform to simulate vehicle motion and flight dynamics 
problems. 

General notations:

The central body is generally referred to as planet and defined by the planet.ini
configuration file. However the central body might be as well set up to be a moon, 
an asteroid or the sun. 

## [Coordinate frames]

Generic coordinate frame conventions used here

### :milky_way: Inertial Frames :milky_way:

PCI - Planet Centered Inertial

### :earth_africa: Central Body Fixed Frames :earth_africa:

PCPF - Planet Centered Inertial

### :rocket: Body fixed Frames :rocket:

B - (General) Body fixed frame

## [Time]

![Sideral Time](https://github.com/maxxonair/rDke/blob/development/assets/images/Siderial_Time.png?raw=true)

Image from https://gssc.esa.int/navipedia/index.php/Sidereal_Time

### UTC

Coordinated Universal Time (UTC) is the time broadcast by WWV and other services. By definition, UTC and TAI have the same rate, but UTC stays close to Mean Solar Time by adding integer numbers of seconds, called leap seconds, from time to time. This keeps solar noon at the same UTC (averaged over the year), even though the rotation of the earth is slowing down.

### GAST

Greenwich Apparent Sidereal Time (GAST) is Greenwich Mean Sidereal Time (GMST) corrected for the shift in the position of the vernal equinox due to nutation.

Ref:

https://lweb.cfa.harvard.edu/~jzhao/times.html 
