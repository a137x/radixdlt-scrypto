use radix_engine_store_interface::interface::{
    CommittableSubstateDatabase, DatabaseUpdates, DbPartitionKey, DbSortKey, DbSubstateValue,
    PartitionEntry, SubstateDatabase,
};
use radix_engine_stores::{
    memory_db::InMemorySubstateDatabase,
    rocks_db::{BlockBasedOptions, Options, RocksdbSubstateStore},
};
use std::{cell::RefCell, collections::BTreeMap, path::PathBuf, time::Duration};

/// Substate store with read time measurements for RocksDB and In Memory DB.
pub struct SubstateStoreWithMetrics<S>
where
    S: SubstateDatabase + CommittableSubstateDatabase,
{
    db: S,
    pub read_metrics: RefCell<BTreeMap<usize, Vec<Duration>>>,
}

impl SubstateStoreWithMetrics<RocksdbSubstateStore> {
    pub fn new_rocksdb(path: PathBuf) -> Self {
        let mut factory_opts = BlockBasedOptions::default();
        factory_opts.disable_cache();

        let mut opt = Options::default();
        opt.set_disable_auto_compactions(true);
        opt.create_if_missing(true);
        opt.create_missing_column_families(true);
        opt.set_block_based_table_factory(&factory_opts);

        Self {
            db: RocksdbSubstateStore::with_options(&opt, path),
            read_metrics: RefCell::new(BTreeMap::new()),
        }
    }
}

impl SubstateStoreWithMetrics<InMemorySubstateDatabase> {
    pub fn new_inmem() -> Self {
        Self {
            db: InMemorySubstateDatabase::standard(),
            read_metrics: RefCell::new(BTreeMap::new()),
        }
    }
}

impl<S: SubstateDatabase + CommittableSubstateDatabase> SubstateDatabase
    for SubstateStoreWithMetrics<S>
{
    fn get_substate(
        &self,
        partition_key: &DbPartitionKey,
        sort_key: &DbSortKey,
    ) -> Option<DbSubstateValue> {
        let start = std::time::Instant::now();
        let ret = self.db.get_substate(partition_key, sort_key);
        let duration = start.elapsed();

        if let Some(value) = ret {
            let exists = self.read_metrics.borrow().get(&value.len()).is_some();
            if exists {
                self.read_metrics
                    .borrow_mut()
                    .get_mut(&value.len())
                    .unwrap()
                    .push(duration);
            } else {
                self.read_metrics
                    .borrow_mut()
                    .insert(value.len(), vec![duration]);
            }
            Some(value)
        } else {
            None
        }
    }

    fn list_entries(
        &self,
        partition_key: &DbPartitionKey,
    ) -> Box<dyn Iterator<Item = PartitionEntry> + '_> {
        self.db.list_entries(partition_key)
    }
}

impl<S: SubstateDatabase + CommittableSubstateDatabase> CommittableSubstateDatabase
    for SubstateStoreWithMetrics<S>
{
    fn commit(&mut self, database_updates: &DatabaseUpdates) {
        self.db.commit(database_updates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blake2::{
        digest::{consts::U32, Digest},
        Blake2b,
    };
    use linreg::linear_regression_of;
    use plotters::prelude::*;
    use radix_engine_store_interface::interface::{
        CommittableSubstateDatabase, DatabaseUpdate, DatabaseUpdates, DbPartitionKey, DbSortKey,
        PartitionUpdates, SubstateDatabase,
    };
    use rand::Rng;
    use std::{io::Write, path::PathBuf};

    /// Range start of the measuremnts
    const MIN_SIZE: usize = 1;
    /// Range end of the measuremnts
    const MAX_SIZE: usize = 4 * 1024 * 1024;
    /// Range step
    const SIZE_STEP: usize = 10 * 1024;
    /// Each step write and read
    const COUNT: usize = 20;
    /// Multiplication of each step read (COUNT * READ_REPEATS)
    const READ_REPEATS: usize = 200;

    #[test]
    /// Database is created in /tmp/radix-scrypto-db folder.
    /// Outputs are genered in png files: /tmp/scrypto_rocksdb_1.png, /tmp/scrypto_inmem_1.png, /tmp/scrypto_diff_1.png
    /// point list is printed to stdout.
    /// To run test casea use command:
    /// cargo test -p radix-engine-utils -p radix-engine-stores --features rocksdb test_store_db --release -- --nocapture
    /// from main radixdlt-scrypto folder.
    fn test_store_db() {
        // RocksDB part
        let path = PathBuf::from(r"/tmp/radix-scrypto-db");
        // clean database
        std::fs::remove_dir_all(path.clone()).ok();

        // prepare database
        let data_index_vector = {
            let mut substate_db = SubstateStoreWithMetrics::new_rocksdb(path.clone());
            prepare_db(&mut substate_db)
        };

        // reopen database
        let mut substate_db = SubstateStoreWithMetrics::new_rocksdb(path);
        run_read_test(&mut substate_db, data_index_vector);

        // prepare data for linear approximation
        drop_edge_values(&mut substate_db);
        let rocksdb_output_data = calculate_percent_to_max_points(&mut substate_db, 95f32);

        // prepare data for plot
        let mut rocksdb_data = Vec::with_capacity(100000);
        for (k, v) in substate_db.read_metrics.borrow().iter() {
            for i in v {
                rocksdb_data.push((*k as f32, i.as_micros() as f32));
            }
        }

        // export results
        export_graph_and_print_summary(
            &mut substate_db,
            "RocksDB random reads",
            &rocksdb_data,
            &rocksdb_output_data,
            "/tmp/scrypto_rocksdb_1.png",
            "95th percentile of reads",
        )
        .unwrap();

        // InMemory DB part
        let mut substate_db = SubstateStoreWithMetrics::new_inmem();
        let data_index_vector = prepare_db(&mut substate_db);
        run_read_test(&mut substate_db, data_index_vector);

        // prepare data for linear approximation
        drop_edge_values(&mut substate_db);
        let inmem_output_data = calculate_percent_to_max_points(&mut substate_db, 95f32);

        // prepare data for plot
        let mut inmem_data = Vec::with_capacity(100000);
        for (k, v) in substate_db.read_metrics.borrow().iter() {
            for i in v {
                inmem_data.push((*k as f32, i.as_micros() as f32));
            }
        }

        // export results
        export_graph_and_print_summary(
            &mut substate_db,
            "InMemoryDB random reads",
            &inmem_data,
            &inmem_output_data,
            "/tmp/scrypto_inmem_1.png",
            "95th percentile of reads",
        )
        .unwrap();

        // Calculate RocksDB - InMemory diff and export results
        export_graph_and_print_summary_for_two_series(
            "RocksDB - InMemoryDB random reads",
            &rocksdb_output_data,
            &inmem_output_data,
            "/tmp/scrypto_diff_1.png",
        )
        .unwrap();
    }

    fn drop_edge_values<S: SubstateDatabase + CommittableSubstateDatabase>(
        substate_store: &mut SubstateStoreWithMetrics<S>,
    ) {
        for (_, v) in substate_store.read_metrics.borrow_mut().iter_mut() {
            v.sort();
            v.pop();
            v.remove(0);
        }
    }

    pub fn calculate_percent_to_max_points<S: SubstateDatabase + CommittableSubstateDatabase>(
        substate_store: &mut SubstateStoreWithMetrics<S>,
        percent: f32,
    ) -> Vec<(f32, f32)> {
        assert!(percent <= 100f32);
        let mut output_values = Vec::new();
        let mut binding = substate_store.read_metrics.borrow_mut();
        for (k, v) in binding.iter_mut() {
            v.sort();
            let idx = (((v.len() - 1) as f32 * percent) / 100f32).round() as usize;
            output_values.push((*k as f32, v[idx].as_micros() as f32));
        }
        output_values
    }

    fn prepare_db<S: SubstateDatabase + CommittableSubstateDatabase>(
        substate_db: &mut S,
    ) -> Vec<(DbPartitionKey, DbSortKey, usize)> {
        let mut data_index_vector: Vec<(DbPartitionKey, DbSortKey, usize)> =
            Vec::with_capacity(MAX_SIZE);

        println!("Preparing database...");
        let mut p_key_cnt = 1u32;
        //let mut substate_db = SubstateStoreWithMetrics::new_rocksdb(path.clone());
        let mut sort_key_value: usize = 0;
        for size in (MIN_SIZE..=MAX_SIZE).step_by(SIZE_STEP) {
            let mut input_data = DatabaseUpdates::new();
            for _ in 0..COUNT {
                let value = DatabaseUpdate::Set(vec![1; size]);

                let plain_bytes = sort_key_value.to_be_bytes().to_vec();
                let mut hashed_prefix: Vec<u8> =
                    Blake2b::<U32>::digest(plain_bytes.clone()).to_vec();
                hashed_prefix.extend(plain_bytes);

                let sort_key = DbSortKey(hashed_prefix);
                sort_key_value += 1;

                let mut partition = PartitionUpdates::new();
                partition.insert(sort_key.clone(), value);

                let partition_key = DbPartitionKey(p_key_cnt.to_be_bytes().to_vec());
                p_key_cnt += 1;

                data_index_vector.push((partition_key.clone(), sort_key, size));

                input_data.insert(partition_key, partition);
            }
            substate_db.commit(&input_data);
        }
        println!("  done\n");

        data_index_vector
    }

    fn run_read_test<S: SubstateDatabase + CommittableSubstateDatabase>(
        substate_db: &mut S,
        data_index_vector: Vec<(DbPartitionKey, DbSortKey, usize)>,
    ) {
        println!("Random read start...");

        let mut rng = rand::thread_rng();

        //let mut p_key_cnt = 1u32;
        for i in 0..READ_REPEATS {
            let time_start = std::time::Instant::now();
            let mut idx_vector: Vec<usize> = (0..data_index_vector.len()).collect();

            for j in 0..data_index_vector.len() {
                assert!(!idx_vector.is_empty());
                let idx = rng.gen_range(0..idx_vector.len());

                let (p, s, v) = &data_index_vector[idx_vector[idx]];

                print!("\rRead {}/{}", j + 1, data_index_vector.len());
                std::io::stdout().flush().ok();

                let read_value = substate_db.get_substate(&p, &s);

                assert!(read_value.is_some());
                assert_eq!(read_value.unwrap().len(), *v);

                idx_vector.remove(idx);
            }

            let time_end = std::time::Instant::now();
            let mut duration = time_end
                .checked_duration_since(time_start)
                .unwrap()
                .as_secs();
            if duration == 0 {
                duration = 1;
            }
            println!(
                "\rRound {}/{}  read time: {} s, left: {} s\r",
                i + 1,
                READ_REPEATS,
                duration,
                (READ_REPEATS - (i + 1)) * duration as usize
            );
        }

        println!("Read done");
    }

    pub fn export_graph_and_print_summary<S: SubstateDatabase + CommittableSubstateDatabase>(
        substate_db: &mut SubstateStoreWithMetrics<S>,
        caption: &str,
        data: &Vec<(f32, f32)>,
        output_data: &Vec<(f32, f32)>,
        output_png_file: &str,
        output_data_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // calculate axis max/min values
        let y_ofs = 10f32;
        let x_ofs = 5000f32;
        let x_min = data.iter().map(|i| (i.0 as i32)).min().unwrap() as f32 - x_ofs;
        let x_max = data.iter().map(|i| (i.0 as i32)).max().unwrap() as f32 + x_ofs;
        let y_min = data.iter().map(|i| i.1 as i32).min().unwrap() as f32 - y_ofs;
        let y_max = data.iter().map(|i| i.1 as i32).max().unwrap() as f32 + y_ofs;

        // 4. calculate linear approximation
        let (lin_slope, lin_intercept): (f64, f64) = linear_regression_of(&output_data).unwrap();
        let lin_x_axis = (x_min as f32..x_max as f32).step(10f32);

        // draw scatter plot
        let root = BitMapBackend::new(output_png_file, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;
        root.margin(20, 20, 20, 20);

        let mut scatter_ctx = ChartBuilder::on(&root)
            .caption(caption, ("sans-serif", 20).into_font())
            .x_label_area_size(40)
            .y_label_area_size(80)
            .margin(20)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
        scatter_ctx
            .configure_mesh()
            .x_desc("Size [bytes]")
            .y_desc("DB read duration [microseconds]")
            .axis_desc_style(("sans-serif", 16))
            .draw()?;
        // 1. draw all read points
        scatter_ctx
            .draw_series(
                data.iter()
                    .map(|(x, y)| Circle::new((*x, *y), 2, GREEN.filled())),
            )?
            .label(format!("Reads (count: {})", data.len()))
            .legend(|(x, y)| Circle::new((x + 10, y), 2, GREEN.filled()));
        // 2. draw median for each read series (basaed on same size)
        scatter_ctx
            .draw_series(
                output_data
                    .iter()
                    .map(|(x, y)| Cross::new((*x, *y), 6, RED)),
            )?
            .label(output_data_name)
            .legend(|(x, y)| Cross::new((x + 10, y), 6, RED));
        // 3. draw linear approximetion line
        scatter_ctx
            .draw_series(LineSeries::new(
                lin_x_axis
                    .values()
                    .map(|x| (x, (lin_slope * x as f64 + lin_intercept) as f32)),
                &BLUE,
            ))?
            .label(format!(
                "Linear approx.: f(x)={:.4}*x+{:.1}",
                lin_slope, lin_intercept
            ))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        scatter_ctx
            .configure_series_labels()
            .background_style(&WHITE)
            .border_style(&BLACK)
            .label_font(("sans-serif", 16))
            .position(SeriesLabelPosition::UpperMiddle)
            .draw()?;

        root.present().expect("Unable to write result to file");

        // print some informations
        println!("Read count: {}", data.len());
        println!(
            "Distinct size read count: {}",
            substate_db.read_metrics.borrow().len()
        );
        println!(
            "Read counts list (size, count): {:?}",
            substate_db
                .read_metrics
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.len()))
                .collect::<Vec<(usize, usize)>>()
        );
        println!(
            "{} points list (size, time): {:?}",
            output_data_name, output_data
        );
        println!(
            "Linear approx.:  f(size) = {} * size + {}\n",
            lin_slope, lin_intercept
        );

        Ok(())
    }

    pub fn export_graph_and_print_summary_for_two_series(
        caption: &str,
        data_series1: &Vec<(f32, f32)>,
        data_series2: &Vec<(f32, f32)>,
        output_png_file: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // calculate diff points
        assert_eq!(data_series1.len(), data_series2.len());
        let mut v1 = data_series1.clone();
        for (idx, (size, diff_value)) in v1.iter_mut().enumerate() {
            assert_eq!(*size, data_series2[idx].0);
            *diff_value -= data_series2[idx].1;
        }
        // calculate linear approximation of diff points
        let (lin_slope, lin_intercept): (f64, f64) = linear_regression_of(&v1).unwrap();

        // calculate linethrough 1st and last diff points
        let v2: Vec<(f32, f32)> = vec![
            *data_series1.first().unwrap(),
            *data_series1.last().unwrap(),
        ];
        let (lin_slope_2, lin_intercept_2): (f64, f64) = linear_regression_of(&v2).unwrap();

        // calculate axis max/min values
        let y_ofs = 10f32;
        let x_ofs = 5000f32;
        let x_min = -x_ofs;
        let x_max = data_series1.iter().map(|i| i.0 as i32).max().unwrap() as f32 + x_ofs;
        let y_min = 0f32;
        let y_max = data_series1.iter().map(|i| i.1 as i32).max().unwrap() as f32 + y_ofs;

        let lin_x_axis = (x_min..x_max).step(10f32);

        // draw scatter plot
        let root = BitMapBackend::new(output_png_file, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;
        root.margin(20, 20, 20, 20);

        let mut scatter_ctx = ChartBuilder::on(&root)
            .caption(caption, ("sans-serif", 20).into_font())
            .x_label_area_size(40)
            .y_label_area_size(80)
            .margin(20)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
        scatter_ctx
            .configure_mesh()
            .x_desc("Size [bytes]")
            .y_desc("DB read duration [microseconds]")
            .axis_desc_style(("sans-serif", 16))
            .draw()?;
        // 1. draw read series1 points
        scatter_ctx
            .draw_series(
                data_series1
                    .iter()
                    .map(|(x, y)| Circle::new((*x, *y), 2, GREEN.filled())),
            )?
            .label("RocksDB read (95th percentile)")
            .legend(|(x, y)| Circle::new((x + 10, y), 2, GREEN.filled()));
        // 2. draw read series2 points
        scatter_ctx
            .draw_series(
                data_series2
                    .iter()
                    .map(|(x, y)| Circle::new((*x, *y), 2, BLUE.filled())),
            )?
            .label("InMemory read (95th percentile)")
            .legend(|(x, y)| Circle::new((x + 10, y), 2, BLUE.filled()));
        // 3. draw read series1-series2 points
        scatter_ctx
            .draw_series(v1.iter().map(|(x, y)| Cross::new((*x, *y), 6, MAGENTA)))?
            .label("Diff points (RocksDB/green - InMemory/blue)")
            .legend(|(x, y)| Cross::new((x + 10, y), 6, MAGENTA));
        // 4. draw linear approximetion line
        scatter_ctx
            .draw_series(LineSeries::new(
                lin_x_axis
                    .values()
                    .map(|x| (x, (lin_slope * x as f64 + lin_intercept) as f32)),
                &RED,
            ))?
            .label(format!(
                "Linear approx. of diff points: f(x)={:.4}*x+{:.1}",
                lin_slope, lin_intercept
            ))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        scatter_ctx
            .draw_series(LineSeries::new(
                lin_x_axis
                    .values()
                    .map(|x| (x, (lin_slope_2 * x as f64 + lin_intercept_2) as f32)),
                &BLACK,
            ))?
            .label(format!(
                "Line by 1st and last RocksDB point: f(x)={:.4}*x+{:.1}",
                lin_slope_2, lin_intercept_2
            ))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));
        scatter_ctx
            .configure_series_labels()
            .background_style(&WHITE)
            .border_style(&BLACK)
            .label_font(("sans-serif", 16))
            .position(SeriesLabelPosition::UpperMiddle)
            .draw()?;

        root.present().expect("Unable to write result to file");

        // print some informations
        println!("Points list (size, time): {:?}", v1);
        println!(
            "Linear approx.:  f(size) = {} * size + {}\n",
            lin_slope, lin_intercept
        );
        println!(
            "Liny by 1st and last RocksDB point:  f(size) = {} * size + {}\n",
            lin_slope_2, lin_intercept_2
        );

        Ok(())
    }
}
