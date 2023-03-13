import random

PARTICLES_NUM = 1000
AREA_LENGTH = 20
PARTICLE_RADIUS = 0.25
COORDS_UPPER_BOUND = 20

with open("dynamic.txt", "w") as f:
    f.write("0\n")
    for i in range(PARTICLES_NUM):
        f.write(
            str(random.uniform(0, COORDS_UPPER_BOUND))
            + " "
            + str(random.uniform(0, COORDS_UPPER_BOUND))
            + "\n"
        )
with open("static.txt", "w") as f:
    f.write(str(PARTICLES_NUM) + "\n")
    f.write(str(AREA_LENGTH) + "\n")
    for i in range(PARTICLES_NUM):
        # All particles share the same radius for now
        f.write(str(PARTICLE_RADIUS) + "\n")
