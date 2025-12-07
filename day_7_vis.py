from matplotlib.animation import FuncAnimation
import numpy as np

import matplotlib.pyplot as plt
import matplotlib.patches as mpatches

# Sample data - replace with your actual paths
paths = [
    [(7, 2), (6, 4), (5, 6), (4, 8), (3, 10), (2, 12), (1, 14), (1, 15)],
    [(7, 2), (6, 4), (5, 6), (4, 8), (3, 10), (2, 12), (1, 14), (1, 15)],
    [(7, 2), (6, 4), (5, 6), (4, 8), (3, 10), (2, 12), (3, 14), (3, 15)],
    [(7, 2), (6, 4), (5, 6), (4, 8), (3, 10), (2, 12), (3, 14), (3, 15)],
    [(7, 2), (6, 4), (5, 6), (4, 8), (3, 10), (3, 15)],
    [(7, 2), (6, 4), (5, 6), (4, 8), (5, 10), (5, 15)],
    [(7, 2), (6, 4), (5, 6), (4, 8), (5, 10), (6, 12), (5, 14), (5, 15)],
    [(7, 2), (6, 4), (5, 6), (4, 8), (5, 10), (6, 12), (5, 14), (5, 15)],
    [(7, 2), (6, 4), (5, 6), (4, 8), (5, 10), (6, 12), (7, 14), (7, 15)],
    [(7, 2), (6, 4), (5, 6), (4, 8), (5, 10), (6, 12), (7, 14), (7, 15)],
    [(7, 2), (6, 4), (5, 6), (6, 8), (5, 10), (5, 15)],
    [(7, 2), (6, 4), (5, 6), (6, 8), (5, 10), (6, 12), (5, 14), (5, 15)],
    [(7, 2), (6, 4), (5, 6), (6, 8), (5, 10), (6, 12), (5, 14), (5, 15)],
    [(7, 2), (6, 4), (5, 6), (6, 8), (5, 10), (6, 12), (7, 14), (7, 15)],
    [(7, 2), (6, 4), (5, 6), (6, 8), (5, 10), (6, 12), (7, 14), (7, 15)],
    [(7, 2), (6, 4), (5, 6), (6, 8), (7, 14), (7, 15)],
    [(7, 2), (6, 4), (5, 6), (6, 8), (7, 14), (7, 15)],
    [(7, 2), (6, 4), (7, 6), (6, 8), (5, 10), (5, 15)],
    [(7, 2), (6, 4), (7, 6), (6, 8), (5, 10), (6, 12), (5, 14), (5, 15)],
    [(7, 2), (6, 4), (7, 6), (6, 8), (5, 10), (6, 12), (5, 14), (5, 15)],
    [(7, 2), (6, 4), (7, 6), (6, 8), (5, 10), (6, 12), (7, 14), (7, 15)],
    [(7, 2), (6, 4), (7, 6), (6, 8), (5, 10), (6, 12), (7, 14), (7, 15)],
    [(7, 2), (6, 4), (7, 6), (6, 8), (7, 14), (7, 15)],
    [(7, 2), (6, 4), (7, 6), (6, 8), (7, 14), (7, 15)],
    [(7, 2), (6, 4), (7, 6), (7, 15)],
    [(7, 2), (8, 4), (7, 6), (6, 8), (5, 10), (5, 15)],
    [(7, 2), (8, 4), (7, 6), (6, 8), (5, 10), (6, 12), (5, 14), (5, 15)],
    [(7, 2), (8, 4), (7, 6), (6, 8), (5, 10), (6, 12), (5, 14), (5, 15)],
    [(7, 2), (8, 4), (7, 6), (6, 8), (5, 10), (6, 12), (7, 14), (7, 15)],
    [(7, 2), (8, 4), (7, 6), (6, 8), (5, 10), (6, 12), (7, 14), (7, 15)],
    [(7, 2), (8, 4), (7, 6), (6, 8), (7, 14), (7, 15)],
    [(7, 2), (8, 4), (7, 6), (6, 8), (7, 14), (7, 15)],
    [(7, 2), (8, 4), (7, 6), (7, 15)],
    [(7, 2), (8, 4), (9, 6), (9, 15)],
    [(7, 2), (8, 4), (9, 6), (10, 8), (9, 10), (9, 15)],
    [(7, 2), (8, 4), (9, 6), (10, 8), (9, 10), (9, 15)],
    [(7, 2), (8, 4), (9, 6), (10, 8), (11, 10), (11, 15)],
    [(7, 2), (8, 4), (9, 6), (10, 8), (11, 10), (12, 12), (12, 15)],
    [(7, 2), (8, 4), (9, 6), (10, 8), (11, 10), (12, 12), (13, 14), (13, 15)],
    [(7, 2), (8, 4), (9, 6), (10, 8), (11, 10), (12, 12), (13, 14), (13, 15)],
]


def visualize_paths_animated(paths, interval=100):
    """Animate through paths one by one with all paths in background"""
    fig, ax = plt.subplots(figsize=(12, 10))

    # Calculate consistent axis limits
    all_x = [p[0] for path in paths for p in path]
    all_y = [p[1] for path in paths for p in path]

    # Plot all paths in background (only once)
    for i, path in enumerate(paths):
        x_coords = [point[0] for point in path]
        y_coords = [point[1] for point in path]
        ax.plot(
            x_coords,
            y_coords,
            "o-",
            alpha=0.15,
            linewidth=1,
            markersize=4,
            color="gray",
        )

    # Create line object for current path
    (line,) = ax.plot([], [], "bo-", linewidth=3, markersize=10, label="Current Path")
    (start_point,) = ax.plot([], [], "go", markersize=15, label="Start")
    (end_point,) = ax.plot([], [], "ro", markersize=15, label="End")

    ax.set_xlabel("X Coordinate", fontsize=12)
    ax.set_ylabel("Y Coordinate", fontsize=12)
    ax.grid(True, alpha=0.3)
    ax.legend()
    ax.invert_yaxis()
    ax.set_xlim(min(all_x) - 1, max(all_x) + 1)
    ax.set_ylim(max(all_y) + 1, min(all_y) - 1)

    def update(frame):
        path = paths[frame]
        x_coords = [point[0] for point in path]
        y_coords = [point[1] for point in path]

        line.set_data(x_coords, y_coords)
        start_point.set_data([x_coords[0]], [y_coords[0]])
        end_point.set_data([x_coords[-1]], [y_coords[-1]])

        ax.set_title(
            f"Path {frame + 1} of {len(paths)}", fontsize=14, fontweight="bold"
        )

        return line, start_point, end_point

    anim = FuncAnimation(fig, update, frames=len(paths), interval=interval, repeat=True)
    plt.show()


# Run visualizations
print(f"Visualizing {len(paths)} paths...")
visualize_paths_animated(paths)
