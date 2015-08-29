import DS from 'ember-data';

export default DS.Model.extend({
  crate: DS.belongsTo('crate'),
});
