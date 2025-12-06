import { Nephrite } from '@nephrite/core';


const nephrite = new Nephrite({
  source: ['src/tokens/**/*.json'],
  platforms: {
    web: {
      transformGroup: 'web',
      buildPath: 'dist/web/',
      files: [
        {
          destination: 'theme.css',
          format: 'css/variables',
        },
      ],
    }
  }
})

nephrite.registerTransform({
  name: 'margin/css/shorthand',
  type: 'value',
  filter: ({ $type }) => $type === 'margin',
  transform: ({ $value }) => {
    const formatMargin = ({ top, right, bottom, left, vertical, horizontal }) => {
      if (vertical && horizontal) return `${vertical} ${horizontal}`.trim();

      return `${top} ${right} ${bottom} ${left}`.trim();
    };

    if (Array.isArray($value)) {
      return $value.map(formatMargin).join(', ');
    }

    if (typeof $value === 'object') {
      return formatMargin($value);
    }

    return $value;
  }
})

nephrite.registerTransformGroup({
  name: 'web',
  transforms: [
    'margin/css/shorthand',
  ]
})

nephrite.registerParser({
  name: 'json',
  pattern: /\.json$/,
  transforms: [
    'margin/css/shorthand',
  ]
})

nephrite.registerAction({
  name: 'hello-word',
  do: () => {
    console.log('Hello, Nephrite!');
  },
  undo: () => {
    console.log('Goodbye, Nephrite!');
  }
})
