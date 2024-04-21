export default interface ModelHyperparameter {
  c: number;
  gamma: "scale" | "auto" | number;
  kernel: "linear" | "poly" | "rbf" | "sigmoid";
  degree?: number;
}
