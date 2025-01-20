<template>
  <v-app>
    <v-main>
      <Puimuri />
    </v-main>
  <v-footer
    class="text-center d-flex flex-column"
  >
    <div>
      <v-btn variant="text" icon="mdi-github" href="https://github.com/braincow/puimuri-trainer"></v-btn>
    </div>

    <div class="pt-0">
      {{  t('footer') }}
    </div>

    <v-divider></v-divider>

    <div>
      &copy; 2025 — <strong>Antti Peltonen</strong>
    </div>
</v-footer>
  </v-app>
</template>

<script setup lang="ts">
  import { useI18n } from 'vue-i18n'

  const { t } = useI18n()
</script>
