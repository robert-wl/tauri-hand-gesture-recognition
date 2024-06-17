<script lang="ts">
  import MenuCard from "../lib/components/home/MenuCard.svelte";
  import { Link } from "svelte-routing";
  import Template from "../lib/components/Template.svelte";
</script>

<Template
  description="A Machine Learning Project to Recognize Hand Gestures, Preprocessed using MediaPipe Framework and Trained using Support Vector Machine Model"
  title="Hand Gesture Recognition">
  <div class="w-full flex flex-wrap justify-center items-center gap-4 p-4">
    <Link to="/preprocess-dataset">
      <MenuCard
        imageUrl="/home/data-preprocess.png"
        text="Preprocess Hand Images with MediaPipe Framework"
        title="Data Preprocessing" />
    </Link>
    <Link to="/training-dataset">
      <MenuCard
        imageUrl="/home/model-training.png"
        text="Train a Support Vector Machine Model to classify Hand Gestures"
        title="Model Training" />
    </Link>
    <Link to="/testing-dataset">
      <MenuCard
        imageUrl="/home/model-testing.png"
        text="View Model Performance Metrics and Confusion Matrix and Test the Model on New Hand Gestures"
        title="Model Testing" />
    </Link>
  </div>
</Template>
