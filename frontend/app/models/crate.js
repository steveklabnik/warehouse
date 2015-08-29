import DS from 'ember-data';

const {
  Model,
  hasMany
} = DS;

export default Model.extend({
   versions: hasMany('version'),
});
