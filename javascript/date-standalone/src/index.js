var dateFns = require('date-fns');

export function main() {
  var date1 = new Date(2021, 4, 12);
  var date2 = new Date(2022, 4, 12);

  return dateFns.formatDistance(date1, date2);
};
