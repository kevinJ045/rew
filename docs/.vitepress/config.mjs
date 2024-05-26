import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Rew",
  description: "A simple coffeescript runtime",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting Started', link: '/getting-started' }
    ],

    sidebar: [
      {
        text: 'Starting',
        items: [
          { text: 'Introduction', link: '/introduction' },
          { text: 'Getting Started', link: '/getting-started' }
        ]
      },
      {
        text: 'Modules',
        items: [
          { text: 'App', link: '/app' },
          { text: 'Runtime', link: '/runtime' }
        ]
      },
      {
        text: 'API',
        items: [
          { text: 'Core', link: '/core' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/kevinj045/rew' }
    ]
  }
})
