import DS from 'ember-data';

export default DS.Model.extend({
  name: DS.attr('string'),

  crate: DS.belongsTo("crate"),
  dependencies: DS.hasMany('dependency'),
});
