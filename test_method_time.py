# Create a test method that will run 100 times per M and read the time
import subprocess

NUM_TESTS = 20
M_UPPER_BOUND = 20
INTERACTION_RANGE = 1


time_per_m = {}
for m in range(1, M_UPPER_BOUND):
    total_time = 0
    for i in range(NUM_TESTS):
        # use subprocess to run the program
        subprocess.run(
            [
                "./target/release/cell_index_method",
                str(INTERACTION_RANGE),
                "-m",
                str(m),
            ]
        )
        with open("output.txt", "r") as f:
            total_time += float(f.readline())
    time_per_m[m] = total_time / NUM_TESTS
with open("time_per_m.txt", "w") as f:
    for m, time in time_per_m.items():
        f.write(f"{m}:  {time} \n")
