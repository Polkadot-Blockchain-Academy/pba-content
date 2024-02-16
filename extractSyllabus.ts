const { globSync } = require('glob')
const { writeFileSync } = require('fs')

// Sorting based on the initial number (even if it is in a string)
const sortArray = (arr: string[]) => {
  const tempArr: any[] = [];
  let n;
  for (let i in arr) {
      tempArr[i] = arr[i].match(/([^0-9]+)|([0-9]+)/g);
      for (let j in tempArr[i]) {
          if( ! isNaN(n = parseInt(tempArr[i][j])) ){
              tempArr[i][j] = n;
          }
      }
  }
  tempArr.sort(function (x, y) {
      for (let i in x) {
          if (y.length < i || x[i] < y[i]) {
              return -1; // x is longer
          }
          if (x[i] > y[i]) {
              return 1;
          }
      }
      return 0;
  });
  for (let i in tempArr) {
      arr[i] = tempArr[i].join('');
  }
  return arr;
}

const dirs = globSync('./syllabus/**/*slides.md', { ignore: ['node_modules/**', '**/README.md'] })
const paths = sortArray(dirs).map((dir:any) =>
  dir.replace("syllabus/", "")
    .replace("_slides.md", "")
    .replace("_Slides.md", "")
    .replace("-slides.md", "")
    .replace("-Slides.md", "")
)


  const parsed: any = {};
  for(let i = 0; i < paths.length; i++) {
    let position = parsed;
    const split = paths[i].split('/');
      for(let j = 0; j < split.length; j++) {
          if(split[j] !== "") {
              if(typeof position[split[j]] === 'undefined')
                  position[split[j]] = {};
              position = position[split[j]];
          }
      }
  }

const data = JSON.stringify(parsed)
writeFileSync('./src/syllabus.json', data)