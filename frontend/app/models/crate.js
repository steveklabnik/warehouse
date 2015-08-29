import DS from 'ember-data';

export default DS.Model.extend({
  versions: DS.hasMany('version'),
});
