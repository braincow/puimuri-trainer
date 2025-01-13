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
        <v-card class="mx-auto" :color="getCardColor(index)" :id="getCardId(index)">
          <v-card-title>{{ t('pre', { DESC: t("exercise.type." + amalgam.exercise.exercise_type)}) }}</v-card-title>
          <v-card-text>
            <p>{{  getQuestion(index) }}</p>
            <v-text-field 
              v-model="amalgam.answer" 
              :label="t('variable.' + amalgam.exercise.missing_variable)" 
              type="number"
              :disabled="amalgam.solution !== undefined"
              @keydown.enter="submitAnswer(index)" 
            ></v-text-field>
          </v-card-text>
          <v-card-actions>
            <v-btn :text="t('button.solve')" @click="submitAnswer(index)" :disabled="amalgam.solution !== undefined || !amalgam.answer"></v-btn>
            <v-btn :text="t('button.show')" :disabled="amalgam.solution === undefined" @click="amalgam.revealed=true"></v-btn>
          </v-card-actions>

          <v-expand-transition>
            <v-card
              v-if="amalgam.revealed"
              class="position-absolute w-100"
              height="100%"
              style="bottom: 0;"
            >
              <v-card-title>{{ t('pre', { DESC: t("exercise.type." + amalgam.exercise.exercise_type)}) }}</v-card-title>
              <v-card-text class="pb-0">
                <p v-if="amalgam.answerCorrect === true">
                  {{ t('correct', { ANSWER: amalgam.answer, UNIT: t('unit.' + amalgam.solution?.unit ) }) }}
                </p>
                <p v-else>
                  {{ t('incorrect', { ANSWER: amalgam.answer, UNIT: t('unit.' + amalgam.solution?.unit) }) }}
                </p>
                <p v-for="step in amalgam.solution?.steps">
                  {{ step }}
                </p>
                <p>
                  {{ t('solved', { ANSWER: amalgam.solution?.answer, UNIT: t('unit.' + amalgam.solution?.unit) }) }}
                </p>
              </v-card-text>

              <v-card-actions>
                <v-btn
                  :text="t('button.close')"
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
  import { ref, onMounted, nextTick } from 'vue';
  import axios from 'axios';
  import { useI18n } from 'vue-i18n'

  const { t } = useI18n()

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

  const getCardId = (id: number) => {
    return "eq_card-" + id
  };

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

      // scroll the new box into view
      await nextTick();
      document.getElementById("eq_card-" + (exercises.value.length - 1))?.scrollIntoView();
    } catch (error) {
      console.error(error);
      loadingError.value=true;
    }
  };

  const submitAnswer = async (index: number) => {
    if (!exercises.value[index].answer) {
      console.log("cowardly refusing to send an empty answer.")
      return;
    }

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

  const getQuestion = (index: number) => {
    const exercise = exercises.value[index].exercise;

    const given = exercise.given_variables;

    if (exercise.exercise_type == EquationExerciseType.OhmsLaw) {
      if (exercise.missing_variable == EquationVariable.Voltage) {
        return t("calculate.ohm.u", {R: given[0][1], I: given[1][1] })
      } else if (exercise.missing_variable == EquationVariable.Current) {
        return t("calculate.ohm.i", {U: given[0][1], R: given[1][1] })
      } else if (exercise.missing_variable == EquationVariable.Resistance) {
        return t("calculate.ohm.r", {U: given[0][1], I: given[1][1] })
      }
    } else if (exercise.exercise_type == EquationExerciseType.Power) {
      if (exercise.missing_variable == EquationVariable.Power) {
        return t("calculate.power.p", {U: given[0][1], I: given[1][1] })
      } else if (exercise.missing_variable == EquationVariable.Voltage) {
        return t("calculate.power.u", {P: given[0][1], I: given[1][1] })
      } else if (exercise.missing_variable == EquationVariable.Current) {
        return t("calculate.power.i", {P: given[0][1], U: given[1][1] })
      }
    } else {
      if (exercise.missing_variable == EquationVariable.Power) {
        return t("calculate.combined.p", {U: given[0][1], R: given[1][1] })
      } else if (exercise.missing_variable == EquationVariable.Current) {
        return t("calculate.combined.i", {P: given[0][1], R: given[1][1] })
      } else if (exercise.missing_variable == EquationVariable.Voltage) {
        return t("calculate.combined.u", {P: given[0][1], R: given[1][1] })
      } else {
        return t("calculate.combined.r", {P: given[0][1], U: given[1][1] })
      }
    }

  }

</script>