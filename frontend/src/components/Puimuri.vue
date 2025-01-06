<template>
  <v-container>
    <v-timeline
      side="end"
    >
      <v-timeline-item
        fill-dot
        dot-color="yellow"
        icon="mdi-help-circle"
      >
        <v-img
        width="400"
          src="@/assets/puimuri.jpg"
        />
      </v-timeline-item>
      <v-timeline-item
        v-for="(amalgam, index) in exercises" 
        :key="index"
        :dot-color="getCardColor(index)"
        fill-dot
        :icon="getCardIcon(index)"
      >
        <v-card class="mx-auto" :color="getCardColor(index)" ref="eq_card-{index}">
          <v-card-title>Harjoitus {{ amalgam.exercise.exercise_type }}</v-card-title>
          <v-card-text>
            <p>
            Ratkaise {{ amalgam.exercise.missing_variable }}, kun 
              <span v-for="(variable, i) in amalgam.exercise.given_variables" :key="i">
                {{ variable[0] }} on {{ variable[1] }} ja
              </span>
            </p>
            <v-text-field 
              v-model="amalgam.answer" 
              :label="amalgam.exercise.missing_variable" 
              type="number"
              :disabled="amalgam.solution !== undefined"
              @keydown.enter="submitAnswer(index)" 
            ></v-text-field>
          </v-card-text>
          <v-card-actions>
            <v-btn text="Ratkaise" @click="submitAnswer(index)" :disabled="amalgam.solution !== undefined"></v-btn>
            <v-btn text="Näytä" :disabled="amalgam.solution === undefined" @click="amalgam.revealed=true"></v-btn>
          </v-card-actions>

          <v-expand-transition>
            <v-card
              v-if="amalgam.revealed"
              class="position-absolute w-100"
              height="100%"
              style="bottom: 0;"
            >
              <v-card-title>Harjoitus {{ amalgam.exercise.exercise_type }}</v-card-title>
              <v-card-text class="pb-0">
                <p>Oikea vastaus {{ amalgam.solution?.answer }}</p>
                <p>Vastauksesi {{ amalgam.answer }} oli
                  <span v-if="amalgam.answerCorrect === true">OIKEIN!</span>
                  <span v-else>VÄÄRIN!</span>
                </p>
                <p v-for="step in amalgam.solution?.steps">
                  {{ step }}
                </p>
              </v-card-text>

              <v-card-actions>
                <v-btn
                  text="Sulje"
                  @click="amalgam.revealed = false"
                ></v-btn>
              </v-card-actions>
            </v-card>
          </v-expand-transition>
        </v-card>
      </v-timeline-item>
    </v-timeline>
  </v-container>
</template>

<script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import axios from 'axios';

  enum EquationExerciseType {
    OhmsLaw = "OhmsLaw",
    Power = "Power",
    Combined = "Combined",
  }

  enum EquationVariable {
    Voltage = "Voltage",
    Current = "Current",
    Resistance = "Resistance",
    Power = "Power",
  }

  interface EquationExercise {
    exercise_type: EquationExerciseType;
    missing_variable: EquationVariable;
    given_variables: [EquationVariable, number][];
    correct_answer?: number; // Optional property for correct_answer
  }

  enum EquationUnit {
    Volt = "Volt",
    Ampere = "Ampere",
    Ohm = "Ohm",
    Watt = "Watt",
  }

  interface EquationExerciseSolution {
    steps: string[];
    answer: number;
    unit: EquationUnit;
  }

  interface EquationExerciseAmalgam {
    exercise: EquationExercise;
    answer: number | undefined;
    answerCorrect: boolean | undefined;
    solution: EquationExerciseSolution | undefined;
    revealed: boolean;
  }

  const exercises = ref<EquationExerciseAmalgam[]>([]);
  const loadingError = ref<boolean>(false);

  const fetchExercise = async () => {
    try {
      const response = await axios.get<EquationExercise>('/api/equation');
      exercises.value.push({
        exercise:  response.data,
        answer: undefined,
        answerCorrect: undefined,
        solution: undefined,
        revealed: false
      });
      loadingError.value=false; 
    } catch (error) {
      console.error(error);
      loadingError.value=true;
    }
  };

  const submitAnswer = async (index: number) => {
    try {
      const response = await axios.post(`/api/equation/answer/${exercises.value[index].answer}`, exercises.value[index].exercise, {
        headers: {
          'Content-Type': 'application/json'
        }
      });

      exercises.value[index].solution = response.data;
      exercises.value[index].answerCorrect = true; // users answer was correct

      fetchExercise();
    } catch (error: any) {
      if (error.response) {
        const response = error.response;
        if (response.status == 412) {
          exercises.value[index].solution = response.data;
          exercises.value[index].answerCorrect = false; // users answer was incorrect

          fetchExercise();
        }
      } else {
        console.error(error);
        loadingError.value=true;
      }
    }
  };

  const getCardColor = (index: number) => {
    if (exercises.value[index].answerCorrect === undefined) {
      return 'white'; // Default color
    } else if (exercises.value[index].answerCorrect) {
      return 'green'; 
    } else {
      return 'red'; 
    }
  };

  const getCardIcon = (index: number) => {
    const exercise = exercises.value[index].exercise;

    if (exercise.exercise_type == EquationExerciseType.OhmsLaw) {
      return "mdi-resistor"
    } else if (exercise.exercise_type == EquationExerciseType.Power) {
      return "mdi-lightning-bolt"
    } else {
      return "mdi-calculator-variant"
    }

  }

  onMounted(fetchExercise); // Fetch the exercise when the component is mounted

</script>