<script lang="ts">
  import MenuCard from "../lib/components/home/MenuCard.svelte";
  import { createTauRPCProxy } from "../../bindings.js";
  import { Link } from "svelte-routing";

  async function test() {
    const tsc = await createTauRPCProxy();
    const res = await tsc.dataset.preprocess_dataset("Dataset BISINDO");
    console.log("FINISHED", res);
  }
</script>

<div class="w-full h-full flex flex-col justify-start items-center gap-4 p-4">
  <div class="flex flex-col py-5 gap-5 items-center">
    <h1 class="text-4xl font-bold text-center mt-16">Hand Gesture Recognition</h1>
    <p class="text-center text-lg max-w-[40rem]">
      A Machine Learning Project to Recognize Hand Gestures, Preprocessed using MediaPipe Framework and Trained using Support Vector Machine Model
    </p>
  </div>
  <div class="w-full flex flex-wrap justify-center items-center gap-4 p-4">
    <Link to="/dataset">
      <MenuCard
        imageUrl="/home/data-preprocess.png"
        text="Preprocess Hand Images with MediaPipe Framework"
        title="Data Preprocessing" />
    </Link>
    <button on:click={test}>
      <MenuCard
        imageUrl="/home/model-training.png"
        text="Train a Support Vector Machine Model with to classify Hand Gestures"
        title="Model Training" />
    </button>
    <MenuCard
      imageUrl="/home/model-testing.png"
      text="Test the Model on New Hand Gestures"
      title="Model Testing" />
  </div>
</div>
