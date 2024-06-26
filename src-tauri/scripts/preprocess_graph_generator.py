import numpy as np
import os
import pandas as pd
import sys
from matplotlib import pyplot as plt
from sklearn.decomposition import PCA
from sklearn.discriminant_analysis import LinearDiscriminantAnalysis as LDA
from sklearn.manifold import TSNE
from sklearn.preprocessing import LabelEncoder, MinMaxScaler


def generate_graphs(csv_path: str, output_path: str) -> None:
    df = pd.read_csv(csv_path)

    if len(df) > 1000:
        df = df.sample(1000)

    x = df.drop(columns=["Image_No", "Label"]).values
    y = df["Label"].values
    classes = df["Label"].unique()


    x_transformed = MinMaxScaler().fit_transform(x)
    y_transformed = LabelEncoder().fit_transform(y)
    data = {}

    pca = PCA(n_components=2)
    data["PCA"] = pca.fit_transform(x_transformed)

    lda = LDA(n_components=2)
    data["LDA"] = lda.fit_transform(x_transformed, y_transformed)

    tsne = TSNE(n_components=3)
    data["TSNE"] = tsne.fit_transform(x_transformed)

    num_classes = len(classes)
    colors = plt.cm.tab20(np.linspace(0, 1, num_classes))


    for key, value in data.items():
        fig, ax = plt.subplots(figsize=(8, 6))
        for i in range(num_classes):
            ax.scatter(data[key][y_transformed == i, 0], data[key][y_transformed == i, 1], label=classes[i], color=colors[i])
        fig.legend(bbox_to_anchor=(1.05, 1), loc='upper left', borderaxespad=0.)
        ax.set_xlabel(f"{key} 1")
        ax.set_ylabel(f"{key} 2")
        plt.tight_layout(rect=(0., 0., 1., 1.))
        fig.suptitle(f'{key} Transformed Data', y=1.02)
        plt.savefig(os.path.join(output_path, f"{key}.png"), bbox_inches='tight')




if __name__ == '__main__':
    csv_path = sys.argv[1]
    output_dir = sys.argv[2]


    generate_graphs(csv_path, output_dir)