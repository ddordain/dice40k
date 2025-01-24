<template>
  <div class="form-container">
    <h1>Attack Simulation</h1>
    <form @submit.prevent="submitForm" class="form">
      <div class="input-group">
        <!-- Dynamically generated input fields -->
        <div class="form-group" v-for="(field, key) in fields" :key="key">
          <label :for="key">{{ field.label }}</label>
          <input
            type="number"
            :id="key"
            v-model.number="formData[key]"
            :step="field.step || 1"
            required
          />
        </div>

        <!-- Checkboxes for additional abilities -->
        <div class="checkbox-group">
          <label>
            <input type="checkbox" v-model="formData.lethal_hit" />
            Lethal Hit
          </label>
          <label>
            <input type="checkbox" v-model="formData.sustained_hit" />
            Sustained Hit
          </label>
          <label>
            <input type="checkbox" v-model="formData.reroll_hit" />
            Reroll Hit
          </label>
          <label>
            <input type="checkbox" v-model="formData.reroll_wound" />
            Reroll Wound
          </label>
        </div>
      </div>

      <button type="submit" class="btn">Simulate</button>
    </form>

    <div v-if="response" class="response">
      <h2>Simulation Results:</h2>
      <p><strong>Average successful attacks:</strong> {{ response.average_successful_attacks }}</p>
      <p><strong>Threshold successful attacks:</strong> {{ response.threshold_successful_attacks }}</p>
    </div>
  </div>
</template>

<script>
import { ref } from "vue";
import axios from "axios";

export default {
  name: "form",
  setup() {
    const formData = ref({
      num_attacks: 8,
      min_to_hit: 2,
      min_to_wound: 2,
      min_to_save: 7,
      threshold_percent: 80.0,
      lethal_hit: false,
      sustained_hit: false,
      reroll_hit: false,
      reroll_wound: false,
    });

    const fields = {
      num_attacks: { label: "Number of attacks:" },
      min_to_hit: { label: "Minimum to hit:" },
      min_to_wound: { label: "Minimum to wound:" },
      min_to_save: { label: "Minimum to save:" },
      threshold_percent: { label: "Threshold percentage:", step: 0.1 },
    };

    const response = ref(null);

    const apiUrl = import.meta.env.MODE === "production"
      ? "https://api.badroll.ddordain/simulate"
      : "http://localhost:8000/simulate"; // Use your dev URL here for local testing

    const submitForm = async () => {
      try {
        const res = await axios.post(apiUrl, formData.value, {
          headers: { "Content-Type": "application/json" },
          withCredentials: false,
        });
        response.value = res.data;
      } catch (error) {
        console.error("Error fetching simulation results:", error);
        alert("Failed to fetch simulation results.");
      }
    };

    return { formData, fields, response, submitForm };
  },
};
</script>

<style>
.form-container {
  background-color: #3b4252; /* nord polar night */
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  text-align: center;
  max-width: 600px;
  margin: auto;
}
h1 {
  color: #81a1c1; /* nord frost */
  font-size: 24px;
  margin-bottom: 20px;
}
.form {
  display: flex;
  flex-direction: column;
  gap: 15px;
}
.form-group {
  display: flex;
  flex-direction: column;
}
input[type="number"] {
  padding: 10px;
  border: 1px solid #4c566a; /* nord polar night */
  background-color: #2e3440; /* nord polar night */
  color: #d8dee9; /* nord snow storm */
  border-radius: 4px;
}
input[type="checkbox"] {
  margin-right: 10px;
}
.checkbox-group {
  display: flex;
  flex-direction: column;
}
button {
  background-color: #81a1c1; /* nord frost */
  color: #2e3440; /* nord polar night */
  padding: 10px;
  border-radius: 4px;
  cursor: pointer;
  font-weight: bold;
  border: none;
  transition: background-color 0.3s;
}
button:hover {
  background-color: #5e81ac; /* nord frost */
}
.response {
  margin-top: 20px;
  background-color: #4c566a; /* nord polar night */
  color: #eceff4; /* nord snow storm */
  padding: 15px;
  border-radius: 4px;
}
.response h2 {
  color: #a3be8c; /* nord frost */
}

/* Mobile responsiveness */
@media (max-width: 600px) {
  .form-container {
    padding: 15px;
  }
  h1 {
    font-size: 20px;
  }
  button {
    padding: 8px;
  }
}
</style>
