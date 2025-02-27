export default {
  navContainer: ({ props }) => ({
    class: [
      // Position
      "relative",

      // Misc
      { "overflow-hidden": props.scrollable },
    ],
  }),
  navContent: ({ instance }) => ({
    class: [
      // Overflow and Scrolling
      "overscroll-contain",
      "overscroll-auto",
      "scroll-smooth",
      "[&::-webkit-scrollbar]:hidden",
    ],
  }),
  previousButton: {
    class: [
      // Flexbox and Alignment
      "flex items-center justify-center",

      // Position
      "!absolute",
      "top-0 left-0",
      "z-20",

      // Size and Shape
      "h-full w-10",
      "rounded-none",

      // Colors
      "bg-surface-0 dark:bg-surface-900",
      "text-surface-700 dark:text-surface-0/80",
      "shadow-sm",
    ],
  },
  nextButton: {
    class: [
      // Flexbox and Alignment
      "flex items-center justify-center",

      // Position
      "!absolute",
      "top-0 right-0",
      "z-20",

      // Size and Shape
      "h-full w-10",
      "rounded-none",

      // Colors
      "text-surface-700 dark:text-surface-0/80",
      "bg-surface-0 dark:bg-surface-900",
      "shadow-sm",
    ],
  },
  nav: {
    class: [
      // Flexbox
      "flex flex-1 gap-0.5",

      // Spacing
      "list-none",
      "p-0 m-0",

      // Colors
      "bg-background",
    ],
  },
  tabpanel: {
    header: ({ props }) => ({
      class: [
        // Spacing
        "mr-0",

        // Misc
        "outline-none",
        {
          "opacity-60 cursor-default user-select-none select-none pointer-events-none": props?.disabled,
        },
      ],
    }),
    headerAction: ({ parent, context }) => ({
      class: [
        "relative",

        // Font
        "font-semibold",

        // Flexbox and Alignment
        "flex items-center",

        // Spacing
        "py-2.5 px-4",
        "-mb-px",

        // Shape
        "rounded-md",

        // Colors and Conditions
        {
          "border-surface-200 dark:border-surface-700": parent.state.d_activeIndex !== context.index,
          "bg-surface-0 dark:bg-surface-900": parent.state.d_activeIndex !== context.index,
          "text-surface-700 dark:text-surface-0/80": parent.state.d_activeIndex !== context.index,

          "bg-surface-0 dark:bg-surface-900": parent.state.d_activeIndex === context.index,
          "bg-card": parent.state.d_activeIndex === context.index,
        },

        // States
        {
          "hover:bg-surface-0 dark:hover:bg-surface-800/80": parent.state.d_activeIndex !== context.index,
          "hover:text-surface-900 dark:hover:text-surface-0": parent.state.d_activeIndex !== context.index,
        },

        // Transitions
        "transition-all duration-200",

        // Misc
        "cursor-pointer select-none text-decoration-none",
        "overflow-hidden",
        "user-select-none",
      ],
    }),
    headerTitle: {
      class: [
        // Text
        "leading-none",
        "whitespace-nowrap",
      ],
    },
  },
  panelcontainer: {
    class: [
      // Shape
      "border-0 rounded-none",
      "border-br-md border-bl-md",

      // Colors
      "bg-surface-0 dark:bg-surface-900",
      "text-surface-900 dark:text-surface-0/80",
    ],
  },
};
