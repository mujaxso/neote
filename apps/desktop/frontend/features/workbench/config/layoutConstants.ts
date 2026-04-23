// Layout constants for the workbench shell
// All sizing policies live here so panels and app shells refer to the same values.

export const LAYOUT = {
  /** Width of the activity rail (px) */
  activityRailWidth: 48,

  /** Left‑side panel (explorer, search, git, debug etc.) */
  panelLeft: {
    minWidth: 180,
    defaultWidth: 280,
    maxWidth: 400,
  },

  /** Right‑side panel (assistant, extensions etc.) */
  panelRight: {
    minWidth: 180,
    defaultWidth: 280,
    maxWidth: 400,
  },

  /** Height of the compact top bar (px) */
  topBarHeight: 40,

  /** Height of the status bar (px) */
  statusBarHeight: 24,

  /**
   * Window width (px) below which side panels are automatically collapsed
   * to protect the editor area.
   */
  collapseThreshold: 700,
} as const;
