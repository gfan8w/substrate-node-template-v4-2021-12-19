//! Benchmarking setup for pallet-template

use super::*;


/*


生成weights.rs:
1）去网上下载模板文件：https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs
2) 把benchmarks的代码准备好，runtime里的依赖也处理好。然后编译 cargo build --release --features runtime-benchmarks
3) 生成bin后，运行：
./target/release/node-template \
benchmark \
--chain dev \
--execution=wasm \
--wasm-execution=compiled \
--pallet pallet-template \
--extrinsic=* \
--steps 50 \
--repeat 20 \
--output ./pallets/template/src/weights.rs \
--template ./tpl/frame-weight-template.hbs

命令输出：

Raw Storage Info
========
Storage: TemplateModule Something (r:0 w:1)

Median Slopes Analysis
========
-- Extrinsic Time --

Model:
Time ~=       20
    + s        0
              µs

Reads = 0 + (0 * s)
Writes = 1 + (0 * s)

Min Squares Analysis
========
-- Extrinsic Time --

Data points distribution:
    s   mean µs  sigma µs       %
    0        19         0    0.0%
    2        19         0    0.0%
    4        19         0    0.0%
    6        19         0    0.0%
    8      19.1       0.3    1.5%
   10        19         0    0.0%
   ......
   86        20         0    0.0%
   88      20.1       0.3    1.4%
   90      20.2       0.4    1.9%
   92        20         0    0.0%
   94      20.4     0.489    2.3%
   96        20         0    0.0%
   98        20         0    0.0%
  100        20         0    0.0%

Quality and confidence:
param     error
s             0

Model:
Time ~=    19.68             <------ 这个值 跟生成的代码 weights.rs 里的 19_682_000 是相近的（* 10^6）
    + s    0.008
              µs

Reads = 0 + (0 * s)
Writes = 1 + (0 * s)


文件生成不全
修改一下 该文件，
补充一个 pub trait WeightInfo { },  该trait 需要再pallet里引用，是一个公用的trait
补充一个 WeightInfo for () 的实现，方便测试环境使用

4)生成一个 weights.rs文件，把这个文件集成到lib.rs里去

5） 参考：
https://zhuanlan.zhihu.com/p/109696123
https://medium.com/oak-blockchain/tutorial-benchmarking-for-parity-substrate-pallet-development-9cb68bf87ea2


*/





#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	do_something {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}


#[cfg(test)]
mod test {
	use super::*;
	use crate::mock::{new_test_ext,Test};
	use frame_support::assert_ok;

	#[test]
	fn it_works_for_Permill_value() {
		new_test_ext().execute_with(|| {
			println!("ok");
			assert_ok!(true)
		});
	}


	#[test]
	fn test_benchmark(){
		new_test_ext().execute_with(||{
			assert_ok!(test_benchmark_do_something::<Test>())
		});
	}

}