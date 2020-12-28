RSpec.describe Rust::HashMap do
  describe "#size" do
    it "should return the number of content" do
      hash = Rust::HashMap.new
      expect(hash.size).to eq(0)
      hash[:foo] = true
      expect(hash.size).to eq(1)
      hash[:bar] = true
      expect(hash.size).to eq(2)
      hash[:foo] = false
      expect(hash.size).to eq(2)
    end
  end

  describe "#[]" do
    it "should return the content" do
      hash = Rust::HashMap.new
      expect(hash[true]).to be_nil
      hash[:foo] = 1
      expect(hash[:foo]).to eq(1)
      hash[:foo] = 2
      expect(hash[:foo]).to eq(2)
    end
  end

  describe "#delete" do
    it "should return the content" do
      hash = Rust::HashMap.new
      expect(hash.delete(:foo)).to be_nil
      hash[:foo] = 1
      expect(hash.delete(:foo)).to eq(1)
      expect(hash.delete(:foo)).to be_nil
    end
  end

  describe "#keys" do
    it "should return all keys" do
      hash = Rust::HashMap.new
      expect(hash.keys).to eq([])
      hash[:foo] = 1
      expect(hash.keys).to eq([:foo])
      hash[:bar] = 2
      expect(hash.keys.sort).to eq([:foo, :bar].sort)
      hash[:foo] = 3
      expect(hash.keys.sort).to eq([:foo, :bar].sort)
    end
  end

  describe "#values" do
    it "should return all keys" do
      hash = Rust::HashMap.new
      expect(hash.values).to eq([])
      hash[:foo] = 1
      expect(hash.values).to eq([1])
      hash[:bar] = 2
      expect(hash.values.sort).to eq([1, 2])
      hash[:foo] = 3
      expect(hash.values.sort).to eq([2, 3].sort)
    end
  end

  it "should support various types" do
    testcases = [
      [true, false],
      [0, 1, -1, 2 ** 60, 2 ** 65, -(2 ** 60), -(2 ** 65)],
      [0.0, 0.5, Math::PI, Math::E],
      [:foo, :bar, :baz],
      ["", "foo", "bar", "baz"],
      [Object.new, Object.new, Object.new],
      [[], [2, 3], {}, { foo: 1 }, "bar", :baz, 1, true, Object.new],
    ]
    testcases.each do |testcase|
      hash = Rust::HashMap.new
      testcase.each_with_index do |key, index|
        hash[key] = index
        expect(hash[key]).to eq(index)
        expect(hash.size).to eq(index + 1)
        hash[key] = index + 1
        expect(hash[key]).to eq(index + 1)
        expect(hash.size).to eq(index + 1)
      end
      testcase.each_with_index do |key, index|
        expect(hash.delete(key)).to eq(index + 1)
        expect(hash.size).to eq(testcase.size - index - 1)
        expect(hash.delete(key)).to eq(nil)
        expect(hash.size).to eq(testcase.size - index - 1)
      end
    end
  end
end
