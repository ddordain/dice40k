<template>
  <div class="form-container">
    <h1>Attack Simulation</h1>
    <form @submit.prevent="submitForm" class="form">
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

      <button type="submit" class="btn">Simulate</button>
    </form>

    <div v-if="response" class="response">
      <h2>Simulation Results:</h2>
      <p><strong>Average Successful Attacks:</strong> {{ response.average_successful_attacks }}</p>
      <p><strong>Threshold Successful Attacks:</strong> {{ response.threshold_successful_attacks }}</p>
    </div>
  </div>
</template>

<script>
import { ref } from "vue";
import axios from "axios";

export default {
  name: "Form",
  setup() {
    const formData = ref({
      num_attacks: 8,
      min_to_hit: 2,
      min_to_wound: 2,
      min_to_save: 7,
      threshold_percent: 80.0,
    });

    const fields = {
      num_attacks: { label: "Number of Attacks:" },
      min_to_hit: { label: "Minimum to Hit:" },
      min_to_wound: { label: "Minimum to Wound:" },
      min_to_save: { label: "Minimum to Save:" },
      threshold_percent: { label: "Threshold Percentage:", step: 0.1 },
    };

    const response = ref(null);

    const submitForm = async () => {
      try {
      const res = await axios.post("http://localhost:8000/simulate", formData.value, {
        headers: { "Content-Type": "application/json" },
        withCredentials: false,
      });
        response.value = res.data;
      } catch (error) {
        console.error("Error while fetching simulation results:", error);
        alert("Failed to fetch simulation results.");
      }
    };

    return { formData, fields, response, submitForm };
  },
};
</script>

<style>
.form-container {
  background-color: #3B4252; /* Nord Polar Night */
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  text-align: center;
}
h1 {
  color: #81A1C1; /* Nord Frost */
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
label {
  font-weight: bold;
  color: #D8DEE9; /* Nord Snow Storm */
  margin-bottom: 5px;
}
input {
  padding: 10px;
  border: 1px solid #4C566A; /* Nord Polar Night */
  background-color: #2E3440; /* Nord Polar Night */
  color: #D8DEE9; /* Nord Snow Storm */
  border-radius: 4px;
}
input:focus {
  outline: none;
  border-color: #88C0D0; /* Nord Frost */
}
.btn {
  background-color: #81A1C1; /* Nord Frost */
  color: #2E3440; /* Nord Polar Night */
  padding: 10px;
  border-radius: 4px;
  cursor: pointer;
  font-weight: bold;
  border: none;
  transition: background-color 0.3s;
}
.btn:hover {
  background-color: #5E81AC; /* Nord Frost */
}
.response {
  margin-top: 20px;
  background-color: #4C566A; /* Nord Polar Night */
  color: #ECEFF4; /* Nord Snow Storm */
  padding: 15px;
  border-radius: 4px;
}
.response h2 {
  color: #A3BE8C; /* Nord Frost */
}
</style>
