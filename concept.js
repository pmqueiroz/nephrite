import { Nephrit } from '@nephrit/core';


const nephrit = new Nephrit({
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

nephrit.registerTransform({
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

nephrit.registerTransformGroup({
  name: 'web',
  transforms: [
    'margin/css/shorthand',
  ]
})

nephrit.registerParser({
  name: 'json',
  pattern: /\.json$/,
  transforms: [
    'margin/css/shorthand',
  ]
})

nephrit.registerAction({
  name: 'hello-word',
  do: () => {
    console.log('Hello, Nephrit!');
  },
  undo: () => {
    console.log('Goodbye, Nephrit!');
  }
})
