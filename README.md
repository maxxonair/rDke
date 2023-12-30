
# Dynamics Kinematics Environment in Rust [rDKE]

:construction: [Under Construction] :construction:

This is the dynamics kinematics simulation environment implemented in Rust. It 
will be a multi-purpose platform to simulate vehicle motion and flight dynamics 
problems. 

General notations:

The central body is generally referred to as planet and defined by the planet.ini
configuration file. However the central body might be as well set up to be a moon, 
an asteroid or the sun. 

Overview

![Simulation framework overview](https://github.com/maxxonair/rDke/blob/development/assets/images/simulation_environment.png?raw=true)

## [Models]

### Overview - Environment models

| Modelling parameter  | Force model  |  Reference |
|---|---|---| 
| Earth gravitational field  | TODO GGM03 model <br> Simplified gravity model without spherical harmonics  |   |
| Third body  | TODO Sun and moon  |   |
| Solar radiation pressure  | TODO cannonball model  |   |
| Atmophere | TODO NRLMSISE-00 atmosphere model <br> Low density model for altitudes of 180 - 500 km   |   |
| Aerodynamic drag/lift  |  TODO continous flow aerodynamic <br> TODO transitional flow <br> Newtonian flow drag for free molecular flow zone |   |

### Overview - Spacecraft models

| Modelling parameter  | Model  |  Reference |
|---|---|---| 
| TODO |   |   |

## [Solver]

The fourth order Runge-Kutta algorithm (rk4) is used to numerically integrate the set of differential equations with a fixed step size (simulation setting parameter). This allows for a computational efficient solving process.  

## [Coordinate frames]

Generic coordinate frame conventions used here

### :milky_way: Inertial Frames :milky_way:

PCI - Planet Centered Inertial

Example inertal reference system for Earth centered simulations -> Celestial Reference System (CRS)

![Inertial reference system](https://github.com/maxxonair/rDke/blob/development/assets/images/crs_frame.png?raw=true)

Image from https://gssc.esa.int/navipedia/index.php/Conventional_Celestial_Reference_System

### :earth_africa: Central Body Fixed Frames :earth_africa:

PCPF - Planet Centered Inertial

### :rocket: Body fixed Frames :rocket:

SBF - Spacecraft Body Frame

## [Time]

![Sideral Time](https://github.com/maxxonair/rDke/blob/development/assets/images/Siderial_Time.png?raw=true)

Image from https://gssc.esa.int/navipedia/index.php/Sidereal_Time

### UTC

Coordinated Universal Time (UTC) is the time broadcast by WWV and other services. By definition, UTC and TAI have the same rate, but UTC stays close to Mean Solar Time by adding integer numbers of seconds, called leap seconds, from time to time. This keeps solar noon at the same UTC (averaged over the year), even though the rotation of the earth is slowing down.

### GAST

Greenwich Apparent Sidereal Time (GAST) is Greenwich Mean Sidereal Time (GMST) corrected for the shift in the position of the vernal equinox due to nutation.

Ref:

https://lweb.cfa.harvard.edu/~jzhao/times.html 
