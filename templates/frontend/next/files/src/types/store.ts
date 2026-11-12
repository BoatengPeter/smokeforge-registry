export type Density = "comfortable" | "compact";

export type PreferencesState = {
  preferences: {
    density: Density;
  };
};

export type AuthFlowState = {
  authFlow: {
    pendingEmail: string | null;
  };
};

export type AppState = PreferencesState & AuthFlowState;
