module.exports = {
  siteMetadata: {
    siteName: `Using Typescript Example`
  },
  plugins: [
    `gatsby-plugin-typescript`,
    // {
    //   resolve: `gatsby-plugin-typography`,
    //   options: {
    //     pathToConfigModule: `src/utils/typography.js`,
    //     omitGoogleFont: true
    //   }
    // },
    `gatsby-transformer-json`,
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        path: `./raw/`
      }
    }
  ]
};
