/* Copyright 2026 Lenovo and C. Pospiech
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * File: LMX_mpitrace_ids.h
 * Description: src/features/basic/LMX_mpitrace_ids.h
 * Author: Dr. Christoph Pospiech
 */

#ifndef SRC_FEATURES_BASIC_LMX_MPITRACE_IDS_H_
#define SRC_FEATURES_BASIC_LMX_MPITRACE_IDS_H_

/**
 * The raw input for this file was created via a perl script
 * `create_mpitrace_ids.pl` in sub directory
 * `src/features/basic/perl` .
 *
 * This script was applied to mpi30-report.txt --- which
 * was created from
 * http://mpi-forum.org/docs/mpi-3.0/mpi30-report.pdf
 * via a call like the following.
 *
 * ~~~~~~~~~~~~~~~~{.bash}
 * pdf2txt.py -o mpi30-report.txt -t text mpi30-report.pdf
 * ~~~~~~~~~~~~~~~~
 *
 * pdf2tyt.py is part of the following Python packet.
 * https://pypi.python.org/pypi/pdf2text/1.0.0
 */

typedef enum {
    /*
     * Chapter 1 Introduction to MPI
     *
     * Chapter 2 MPI Terms and Conventions
     *
     * Chapter 3 Point-to-Point Communication
     */
    mpi_send_id = 0,                      /**< Id for MPI_Send (chapter 3) */
    mpi_recv_id,                          /**< Id for MPI_Recv (chapter 3) */
    mpi_get_count_id,                     /**< Id for MPI_Get_count (chapter 3) */
    mpi_bsend_id,                         /**< Id for MPI_Bsend (chapter 3) */
    mpi_ssend_id,                         /**< Id for MPI_Ssend (chapter 3) */
    mpi_rsend_id,                         /**< Id for MPI_Rsend (chapter 3) */
    mpi_buffer_attach_id,                 /**< Id for MPI_Buffer_attach (chapter 3) */
    mpi_buffer_detach_id,                 /**< Id for MPI_Buffer_detach (chapter 3) */
    mpi_isend_id,                         /**< Id for MPI_Isend (chapter 3) */
    mpi_ibsend_id,                        /**< Id for MPI_Ibsend (chapter 3) */
    mpi_issend_id,                        /**< Id for MPI_Issend (chapter 3) */
    mpi_irsend_id,                        /**< Id for MPI_Irsend (chapter 3) */
    mpi_irecv_id,                         /**< Id for MPI_Irecv (chapter 3) */
    mpi_wait_id,                          /**< Id for MPI_Wait (chapter 3) */
    mpi_test_id,                          /**< Id for MPI_Test (chapter 3) */
    mpi_request_free_id,                  /**< Id for MPI_Request_free (chapter 3) */
    mpi_waitany_id,                       /**< Id for MPI_Waitany (chapter 3) */
    mpi_testany_id,                       /**< Id for MPI_Testany (chapter 3) */
    mpi_waitall_id,                       /**< Id for MPI_Waitall (chapter 3) */
    mpi_testall_id,                       /**< Id for MPI_Testall (chapter 3) */
    mpi_waitsome_id,                      /**< Id for MPI_Waitsome (chapter 3) */
    mpi_testsome_id,                      /**< Id for MPI_Testsome (chapter 3) */
    mpi_request_get_status_id,            /**< Id for MPI_Request_get_status (chapter 3) */
    mpi_iprobe_id,                        /**< Id for MPI_Iprobe (chapter 3) */
    mpi_probe_id,                         /**< Id for MPI_Probe (chapter 3) */
    mpi_improbe_id,                       /**< Id for MPI_Improbe (chapter 3) */
    mpi_mprobe_id,                        /**< Id for MPI_Mprobe (chapter 3) */
    mpi_mrecv_id,                         /**< Id for MPI_Mrecv (chapter 3) */
    mpi_imrecv_id,                        /**< Id for MPI_Imrecv (chapter 3) */
    mpi_cancel_id,                        /**< Id for MPI_Cancel (chapter 3) */
    mpi_test_cancelled_id,                /**< Id for MPI_Test_cancelled (chapter 3) */
    mpi_send_init_id,                     /**< Id for MPI_Send_init (chapter 3) */
    mpi_bsend_init_id,                    /**< Id for MPI_Bsend_init (chapter 3) */
    mpi_ssend_init_id,                    /**< Id for MPI_Ssend_init (chapter 3) */
    mpi_rsend_init_id,                    /**< Id for MPI_Rsend_init (chapter 3) */
    mpi_recv_init_id,                     /**< Id for MPI_Recv_init (chapter 3) */
    mpi_start_id,                         /**< Id for MPI_Start (chapter 3) */
    mpi_startall_id,                      /**< Id for MPI_Startall (chapter 3) */
    mpi_sendrecv_id,                      /**< Id for MPI_Sendrecv (chapter 3) */
    mpi_sendrecv_replace_id,              /**< Id for MPI_Sendrecv_replace (chapter 3) */
                                          /*
                                           * Chapter 4 Datatypes
                                           */
    mpi_type_contiguous_id,               /**< Id for MPI_Type_contiguous (chapter 4) */
    mpi_type_vector_id,                   /**< Id for MPI_Type_vector (chapter 4) */
    mpi_type_create_hvector_id,           /**< Id for MPI_Type_create_hvector (chapter 4) */
    mpi_type_indexed_id,                  /**< Id for MPI_Type_indexed (chapter 4) */
    mpi_type_create_hindexed_id,          /**< Id for MPI_Type_create_hindexed (chapter 4) */
    mpi_type_create_indexed_block_id,     /**< Id for MPI_Type_create_indexed_block (chapter 4) */
    mpi_type_create_hindexed_block_id,    /**< Id for MPI_Type_create_hindexed_block (chapter 4) */
    mpi_type_create_struct_id,            /**< Id for MPI_Type_create_struct (chapter 4) */
    mpi_type_create_subarray_id,          /**< Id for MPI_Type_create_subarray (chapter 4) */
    mpi_type_create_darray_id,            /**< Id for MPI_Type_create_darray (chapter 4) */
    mpi_get_address_id,                   /**< Id for MPI_Get_address (chapter 4) */
    mpi_type_size_id,                     /**< Id for MPI_Type_size (chapter 4) */
    mpi_type_size_x_id,                   /**< Id for MPI_Type_size_x (chapter 4) */
    mpi_type_get_extent_id,               /**< Id for MPI_Type_get_extent (chapter 4) */
    mpi_type_get_extent_x_id,             /**< Id for MPI_Type_get_extent_x (chapter 4) */
    mpi_type_create_resized_id,           /**< Id for MPI_Type_create_resized (chapter 4) */
    mpi_type_get_true_extent_id,          /**< Id for MPI_Type_get_true_extent (chapter 4) */
    mpi_type_get_true_extent_x_id,        /**< Id for MPI_Type_get_true_extent_x (chapter 4) */
    mpi_type_commit_id,                   /**< Id for MPI_Type_commit (chapter 4) */
    mpi_type_free_id,                     /**< Id for MPI_Type_free (chapter 4) */
    mpi_type_dup_id,                      /**< Id for MPI_Type_dup (chapter 4) */
    mpi_get_elements_id,                  /**< Id for MPI_Get_elements (chapter 4) */
    mpi_get_elements_x_id,                /**< Id for MPI_Get_elements_x (chapter 4) */
    mpi_type_get_envelope_id,             /**< Id for MPI_Type_get_envelope (chapter 4) */
    mpi_type_get_contents_id,             /**< Id for MPI_Type_get_contents (chapter 4) */
    mpi_pack_id,                          /**< Id for MPI_Pack (chapter 4) */
    mpi_unpack_id,                        /**< Id for MPI_Unpack (chapter 4) */
    mpi_pack_size_id,                     /**< Id for MPI_Pack_size (chapter 4) */
    mpi_pack_external_id,                 /**< Id for MPI_Pack_external (chapter 4) */
    mpi_unpack_external_id,               /**< Id for MPI_Unpack_external (chapter 4) */
    mpi_pack_external_size_id,            /**< Id for MPI_Pack_external_size (chapter 4) */
                                          /*
                                           * Chapter 5 Collective Communication
                                           */
    mpi_barrier_id,                       /**< Id for MPI_Barrier (chapter 5) */
    mpi_bcast_id,                         /**< Id for MPI_Bcast (chapter 5) */
    mpi_gather_id,                        /**< Id for MPI_Gather (chapter 5) */
    mpi_gatherv_id,                       /**< Id for MPI_Gatherv (chapter 5) */
    mpi_scatter_id,                       /**< Id for MPI_Scatter (chapter 5) */
    mpi_scatterv_id,                      /**< Id for MPI_Scatterv (chapter 5) */
    mpi_allgather_id,                     /**< Id for MPI_Allgather (chapter 5) */
    mpi_allgatherv_id,                    /**< Id for MPI_Allgatherv (chapter 5) */
    mpi_alltoall_id,                      /**< Id for MPI_Alltoall (chapter 5) */
    mpi_alltoallv_id,                     /**< Id for MPI_Alltoallv (chapter 5) */
    mpi_alltoallw_id,                     /**< Id for MPI_Alltoallw (chapter 5) */
    mpi_reduce_id,                        /**< Id for MPI_Reduce (chapter 5) */
    mpi_op_create_id,                     /**< Id for MPI_Op_create (chapter 5) */
    mpi_op_free_id,                       /**< Id for MPI_Op_free (chapter 5) */
    mpi_allreduce_id,                     /**< Id for MPI_Allreduce (chapter 5) */
    mpi_reduce_local_id,                  /**< Id for MPI_Reduce_local (chapter 5) */
    mpi_op_commutative_id,                /**< Id for MPI_Op_commutative (chapter 5) */
    mpi_reduce_scatter_block_id,          /**< Id for MPI_Reduce_scatter_block (chapter 5) */
    mpi_reduce_scatter_id,                /**< Id for MPI_Reduce_scatter (chapter 5) */
    mpi_scan_id,                          /**< Id for MPI_Scan (chapter 5) */
    mpi_exscan_id,                        /**< Id for MPI_Exscan (chapter 5) */
    mpi_ibarrier_id,                      /**< Id for MPI_Ibarrier (chapter 5) */
    mpi_ibcast_id,                        /**< Id for MPI_Ibcast (chapter 5) */
    mpi_igather_id,                       /**< Id for MPI_Igather (chapter 5) */
    mpi_igatherv_id,                      /**< Id for MPI_Igatherv (chapter 5) */
    mpi_iscatter_id,                      /**< Id for MPI_Iscatter (chapter 5) */
    mpi_iscatterv_id,                     /**< Id for MPI_Iscatterv (chapter 5) */
    mpi_iallgather_id,                    /**< Id for MPI_Iallgather (chapter 5) */
    mpi_iallgatherv_id,                   /**< Id for MPI_Iallgatherv (chapter 5) */
    mpi_ialltoall_id,                     /**< Id for MPI_Ialltoall (chapter 5) */
    mpi_ialltoallv_id,                    /**< Id for MPI_Ialltoallv (chapter 5) */
    mpi_ialltoallw_id,                    /**< Id for MPI_Ialltoallw (chapter 5) */
    mpi_ireduce_id,                       /**< Id for MPI_Ireduce (chapter 5) */
    mpi_iallreduce_id,                    /**< Id for MPI_Iallreduce (chapter 5) */
    mpi_ireduce_scatter_block_id,         /**< Id for MPI_Ireduce_scatter_block (chapter 5) */
    mpi_ireduce_scatter_id,               /**< Id for MPI_Ireduce_scatter (chapter 5) */
    mpi_iscan_id,                         /**< Id for MPI_Iscan (chapter 5) */
    mpi_iexscan_id,                       /**< Id for MPI_Iexscan (chapter 5) */
                                          /*
                                           * Chapter 6 Groups, Contexts, Communicators,
                                           */
    mpi_group_size_id,                    /**< Id for MPI_Group_size (chapter 6) */
    mpi_group_rank_id,                    /**< Id for MPI_Group_rank (chapter 6) */
    mpi_group_translate_ranks_id,         /**< Id for MPI_Group_translate_ranks (chapter 6) */
    mpi_group_compare_id,                 /**< Id for MPI_Group_compare (chapter 6) */
    mpi_comm_group_id,                    /**< Id for MPI_Comm_group (chapter 6) */
    mpi_group_union_id,                   /**< Id for MPI_Group_union (chapter 6) */
    mpi_group_intersection_id,            /**< Id for MPI_Group_intersection (chapter 6) */
    mpi_group_difference_id,              /**< Id for MPI_Group_difference (chapter 6) */
    mpi_group_incl_id,                    /**< Id for MPI_Group_incl (chapter 6) */
    mpi_group_excl_id,                    /**< Id for MPI_Group_excl (chapter 6) */
    mpi_group_range_incl_id,              /**< Id for MPI_Group_range_incl (chapter 6) */
    mpi_group_range_excl_id,              /**< Id for MPI_Group_range_excl (chapter 6) */
    mpi_group_free_id,                    /**< Id for MPI_Group_free (chapter 6) */
    mpi_comm_size_id,                     /**< Id for MPI_Comm_size (chapter 6) */
    mpi_comm_rank_id,                     /**< Id for MPI_Comm_rank (chapter 6) */
    mpi_comm_compare_id,                  /**< Id for MPI_Comm_compare (chapter 6) */
    mpi_comm_dup_id,                      /**< Id for MPI_Comm_dup (chapter 6) */
    mpi_comm_dup_with_info_id,            /**< Id for MPI_Comm_dup_with_info (chapter 6) */
    mpi_comm_idup_id,                     /**< Id for MPI_Comm_idup (chapter 6) */
    mpi_comm_create_id,                   /**< Id for MPI_Comm_create (chapter 6) */
    mpi_comm_create_group_id,             /**< Id for MPI_Comm_create_group (chapter 6) */
    mpi_comm_split_id,                    /**< Id for MPI_Comm_split (chapter 6) */
    mpi_comm_split_type_id,               /**< Id for MPI_Comm_split_type (chapter 6) */
    mpi_comm_free_id,                     /**< Id for MPI_Comm_free (chapter 6) */
    mpi_comm_set_info_id,                 /**< Id for MPI_Comm_set_info (chapter 6) */
    mpi_comm_get_info_id,                 /**< Id for MPI_Comm_get_info (chapter 6) */
    mpi_comm_test_inter_id,               /**< Id for MPI_Comm_test_inter (chapter 6) */
    mpi_comm_remote_size_id,              /**< Id for MPI_Comm_remote_size (chapter 6) */
    mpi_comm_remote_group_id,             /**< Id for MPI_Comm_remote_group (chapter 6) */
    mpi_intercomm_create_id,              /**< Id for MPI_Intercomm_create (chapter 6) */
    mpi_intercomm_merge_id,               /**< Id for MPI_Intercomm_merge (chapter 6) */
    mpi_comm_create_keyval_id,            /**< Id for MPI_Comm_create_keyval (chapter 6) */
    mpi_comm_free_keyval_id,              /**< Id for MPI_Comm_free_keyval (chapter 6) */
    mpi_comm_set_attr_id,                 /**< Id for MPI_Comm_set_attr (chapter 6) */
    mpi_comm_get_attr_id,                 /**< Id for MPI_Comm_get_attr (chapter 6) */
    mpi_comm_delete_attr_id,              /**< Id for MPI_Comm_delete_attr (chapter 6) */
    mpi_win_create_keyval_id,             /**< Id for MPI_Win_create_keyval (chapter 6) */
    mpi_win_free_keyval_id,               /**< Id for MPI_Win_free_keyval (chapter 6) */
    mpi_win_set_attr_id,                  /**< Id for MPI_Win_set_attr (chapter 6) */
    mpi_win_get_attr_id,                  /**< Id for MPI_Win_get_attr (chapter 6) */
    mpi_win_delete_attr_id,               /**< Id for MPI_Win_delete_attr (chapter 6) */
    mpi_type_create_keyval_id,            /**< Id for MPI_Type_create_keyval (chapter 6) */
    mpi_type_free_keyval_id,              /**< Id for MPI_Type_free_keyval (chapter 6) */
    mpi_type_set_attr_id,                 /**< Id for MPI_Type_set_attr (chapter 6) */
    mpi_type_get_attr_id,                 /**< Id for MPI_Type_get_attr (chapter 6) */
    mpi_type_delete_attr_id,              /**< Id for MPI_Type_delete_attr (chapter 6) */
    mpi_comm_set_name_id,                 /**< Id for MPI_Comm_set_name (chapter 6) */
    mpi_comm_get_name_id,                 /**< Id for MPI_Comm_get_name (chapter 6) */
    mpi_type_set_name_id,                 /**< Id for MPI_Type_set_name (chapter 6) */
    mpi_type_get_name_id,                 /**< Id for MPI_Type_get_name (chapter 6) */
    mpi_win_set_name_id,                  /**< Id for MPI_Win_set_name (chapter 6) */
    mpi_win_get_name_id,                  /**< Id for MPI_Win_get_name (chapter 6) */
                                          /*
                                           * Chapter 7 Process Topologies
                                           */
    mpi_cart_create_id,                   /**< Id for MPI_Cart_create (chapter 7) */
    mpi_dims_create_id,                   /**< Id for MPI_Dims_create (chapter 7) */
    mpi_graph_create_id,                  /**< Id for MPI_Graph_create (chapter 7) */
    mpi_dist_graph_create_adjacent_id,    /**< Id for MPI_Dist_graph_create_adjacent (chapter 7) */
    mpi_dist_graph_create_id,             /**< Id for MPI_Dist_graph_create (chapter 7) */
    mpi_topo_test_id,                     /**< Id for MPI_Topo_test (chapter 7) */
    mpi_graphdims_get_id,                 /**< Id for MPI_Graphdims_get (chapter 7) */
    mpi_graph_get_id,                     /**< Id for MPI_Graph_get (chapter 7) */
    mpi_cartdim_get_id,                   /**< Id for MPI_Cartdim_get (chapter 7) */
    mpi_cart_get_id,                      /**< Id for MPI_Cart_get (chapter 7) */
    mpi_cart_rank_id,                     /**< Id for MPI_Cart_rank (chapter 7) */
    mpi_cart_coords_id,                   /**< Id for MPI_Cart_coords (chapter 7) */
    mpi_graph_neighbors_count_id,         /**< Id for MPI_Graph_neighbors_count (chapter 7) */
    mpi_graph_neighbors_id,               /**< Id for MPI_Graph_neighbors (chapter 7) */
    mpi_dist_graph_neighbors_count_id,    /**< Id for MPI_Dist_graph_neighbors_count (chapter 7) */
    mpi_dist_graph_neighbors_id,          /**< Id for MPI_Dist_graph_neighbors (chapter 7) */
    mpi_cart_shift_id,                    /**< Id for MPI_Cart_shift (chapter 7) */
    mpi_cart_sub_id,                      /**< Id for MPI_Cart_sub (chapter 7) */
    mpi_cart_map_id,                      /**< Id for MPI_Cart_map (chapter 7) */
    mpi_graph_map_id,                     /**< Id for MPI_Graph_map (chapter 7) */
    mpi_neighbor_allgather_id,            /**< Id for MPI_Neighbor_allgather (chapter 7) */
    mpi_neighbor_allgatherv_id,           /**< Id for MPI_Neighbor_allgatherv (chapter 7) */
    mpi_neighbor_alltoall_id,             /**< Id for MPI_Neighbor_alltoall (chapter 7) */
    mpi_neighbor_alltoallv_id,            /**< Id for MPI_Neighbor_alltoallv (chapter 7) */
    mpi_neighbor_alltoallw_id,            /**< Id for MPI_Neighbor_alltoallw (chapter 7) */
    mpi_ineighbor_allgather_id,           /**< Id for MPI_Ineighbor_allgather (chapter 7) */
    mpi_ineighbor_allgatherv_id,          /**< Id for MPI_Ineighbor_allgatherv (chapter 7) */
    mpi_ineighbor_alltoall_id,            /**< Id for MPI_Ineighbor_alltoall (chapter 7) */
    mpi_ineighbor_alltoallv_id,           /**< Id for MPI_Ineighbor_alltoallv (chapter 7) */
    mpi_ineighbor_alltoallw_id,           /**< Id for MPI_Ineighbor_alltoallw (chapter 7) */
                                          /*
                                           * Chapter 8 MPI Environmental Management
                                           */
    mpi_get_version_id,                   /**< Id for MPI_Get_version (chapter 8) */
    mpi_get_library_version_id,           /**< Id for MPI_Get_library_version (chapter 8) */
    mpi_get_processor_name_id,            /**< Id for MPI_Get_processor_name (chapter 8) */
    mpi_alloc_mem_id,                     /**< Id for MPI_Alloc_mem (chapter 8) */
    mpi_free_mem_id,                      /**< Id for MPI_Free_mem (chapter 8) */
    mpi_comm_create_errhandler_id,        /**< Id for MPI_Comm_create_errhandler (chapter 8) */
    mpi_comm_set_errhandler_id,           /**< Id for MPI_Comm_set_errhandler (chapter 8) */
    mpi_comm_get_errhandler_id,           /**< Id for MPI_Comm_get_errhandler (chapter 8) */
    mpi_win_create_errhandler_id,         /**< Id for MPI_Win_create_errhandler (chapter 8) */
    mpi_win_set_errhandler_id,            /**< Id for MPI_Win_set_errhandler (chapter 8) */
    mpi_win_get_errhandler_id,            /**< Id for MPI_Win_get_errhandler (chapter 8) */
    mpi_file_create_errhandler_id,        /**< Id for MPI_File_create_errhandler (chapter 8) */
    mpi_file_set_errhandler_id,           /**< Id for MPI_File_set_errhandler (chapter 8) */
    mpi_file_get_errhandler_id,           /**< Id for MPI_File_get_errhandler (chapter 8) */
    mpi_errhandler_free_id,               /**< Id for MPI_Errhandler_free (chapter 8) */
    mpi_error_string_id,                  /**< Id for MPI_Error_string (chapter 8) */
    mpi_error_class_id,                   /**< Id for MPI_Error_class (chapter 8) */
    mpi_add_error_class_id,               /**< Id for MPI_Add_error_class (chapter 8) */
    mpi_add_error_code_id,                /**< Id for MPI_Add_error_code (chapter 8) */
    mpi_add_error_string_id,              /**< Id for MPI_Add_error_string (chapter 8) */
    mpi_comm_call_errhandler_id,          /**< Id for MPI_Comm_call_errhandler (chapter 8) */
    mpi_win_call_errhandler_id,           /**< Id for MPI_Win_call_errhandler (chapter 8) */
    mpi_file_call_errhandler_id,          /**< Id for MPI_File_call_errhandler (chapter 8) */
    mpi_initialized_id,                   /**< Id for MPI_Initialized (chapter 8) */
    mpi_abort_id,                         /**< Id for MPI_Abort (chapter 8) */
    mpi_finalized_id,                     /**< Id for MPI_Finalized (chapter 8) */
                                          /*
                                           * Chapter 9 The Info Object
                                           */
    mpi_info_create_id,                   /**< Id for MPI_Info_create (chapter 9) */
    mpi_info_set_id,                      /**< Id for MPI_Info_set (chapter 9) */
    mpi_info_delete_id,                   /**< Id for MPI_Info_delete (chapter 9) */
    mpi_info_get_id,                      /**< Id for MPI_Info_get (chapter 9) */
    mpi_info_get_valuelen_id,             /**< Id for MPI_Info_get_valuelen (chapter 9) */
    mpi_info_get_nkeys_id,                /**< Id for MPI_Info_get_nkeys (chapter 9) */
    mpi_info_get_nthkey_id,               /**< Id for MPI_Info_get_nthkey (chapter 9) */
    mpi_info_dup_id,                      /**< Id for MPI_Info_dup (chapter 9) */
    mpi_info_free_id,                     /**< Id for MPI_Info_free (chapter 9) */
                                          /*
                                           * Chapter 10 Process Creation and Management
                                           */
    mpi_comm_spawn_id,                    /**< Id for MPI_Comm_spawn (chapter 10) */
    mpi_comm_get_parent_id,               /**< Id for MPI_Comm_get_parent (chapter 10) */
    mpi_comm_spawn_multiple_id,           /**< Id for MPI_Comm_spawn_multiple (chapter 10) */
    mpi_open_port_id,                     /**< Id for MPI_Open_port (chapter 10) */
    mpi_close_port_id,                    /**< Id for MPI_Close_port (chapter 10) */
    mpi_comm_accept_id,                   /**< Id for MPI_Comm_accept (chapter 10) */
    mpi_comm_connect_id,                  /**< Id for MPI_Comm_connect (chapter 10) */
    mpi_publish_name_id,                  /**< Id for MPI_Publish_name (chapter 10) */
    mpi_unpublish_name_id,                /**< Id for MPI_Unpublish_name (chapter 10) */
    mpi_lookup_name_id,                   /**< Id for MPI_Lookup_name (chapter 10) */
    mpi_comm_disconnect_id,               /**< Id for MPI_Comm_disconnect (chapter 10) */
    mpi_comm_join_id,                     /**< Id for MPI_Comm_join (chapter 10) */
                                          /*
                                           * Chapter 11 One-Sided Communications
                                           */
    mpi_win_create_id,                    /**< Id for MPI_Win_create (chapter 11) */
    mpi_win_allocate_id,                  /**< Id for MPI_Win_allocate (chapter 11) */
    mpi_win_allocate_shared_id,           /**< Id for MPI_Win_allocate_shared (chapter 11) */
    mpi_win_shared_query_id,              /**< Id for MPI_Win_shared_query (chapter 11) */
    mpi_win_create_dynamic_id,            /**< Id for MPI_Win_create_dynamic (chapter 11) */
    mpi_win_attach_id,                    /**< Id for MPI_Win_attach (chapter 11) */
    mpi_win_detach_id,                    /**< Id for MPI_Win_detach (chapter 11) */
    mpi_win_free_id,                      /**< Id for MPI_Win_free (chapter 11) */
    mpi_win_get_group_id,                 /**< Id for MPI_Win_get_group (chapter 11) */
    mpi_win_set_info_id,                  /**< Id for MPI_Win_set_info (chapter 11) */
    mpi_win_get_info_id,                  /**< Id for MPI_Win_get_info (chapter 11) */
    mpi_put_id,                           /**< Id for MPI_Put (chapter 11) */
    mpi_get_id,                           /**< Id for MPI_Get (chapter 11) */
    mpi_accumulate_id,                    /**< Id for MPI_Accumulate (chapter 11) */
    mpi_get_accumulate_id,                /**< Id for MPI_Get_accumulate (chapter 11) */
    mpi_fetch_and_op_id,                  /**< Id for MPI_Fetch_and_op (chapter 11) */
    mpi_compare_and_swap_id,              /**< Id for MPI_Compare_and_swap (chapter 11) */
    mpi_rput_id,                          /**< Id for MPI_Rput (chapter 11) */
    mpi_rget_id,                          /**< Id for MPI_Rget (chapter 11) */
    mpi_raccumulate_id,                   /**< Id for MPI_Raccumulate (chapter 11) */
    mpi_rget_accumulate_id,               /**< Id for MPI_Rget_accumulate (chapter 11) */
    mpi_win_fence_id,                     /**< Id for MPI_Win_fence (chapter 11) */
    mpi_win_start_id,                     /**< Id for MPI_Win_start (chapter 11) */
    mpi_win_complete_id,                  /**< Id for MPI_Win_complete (chapter 11) */
    mpi_win_post_id,                      /**< Id for MPI_Win_post (chapter 11) */
    mpi_win_wait_id,                      /**< Id for MPI_Win_wait (chapter 11) */
    mpi_win_test_id,                      /**< Id for MPI_Win_test (chapter 11) */
    mpi_win_lock_id,                      /**< Id for MPI_Win_lock (chapter 11) */
    mpi_win_lock_all_id,                  /**< Id for MPI_Win_lock_all (chapter 11) */
    mpi_win_unlock_id,                    /**< Id for MPI_Win_unlock (chapter 11) */
    mpi_win_unlock_all_id,                /**< Id for MPI_Win_unlock_all (chapter 11) */
    mpi_win_flush_id,                     /**< Id for MPI_Win_flush (chapter 11) */
    mpi_win_flush_all_id,                 /**< Id for MPI_Win_flush_all (chapter 11) */
    mpi_win_flush_local_id,               /**< Id for MPI_Win_flush_local (chapter 11) */
    mpi_win_flush_local_all_id,           /**< Id for MPI_Win_flush_local_all (chapter 11) */
    mpi_win_sync_id,                      /**< Id for MPI_Win_sync (chapter 11) */
                                          /*
                                           * Chapter 12 External Interfaces
                                           */
    mpi_grequest_start_id,                /**< Id for MPI_Grequest_start (chapter 12) */
    mpi_grequest_complete_id,             /**< Id for MPI_Grequest_complete (chapter 12) */
    mpi_status_set_elements_id,           /**< Id for MPI_Status_set_elements (chapter 12) */
    mpi_status_set_elements_x_id,         /**< Id for MPI_Status_set_elements_x (chapter 12) */
    mpi_status_set_cancelled_id,          /**< Id for MPI_Status_set_cancelled (chapter 12) */
    mpi_query_thread_id,                  /**< Id for MPI_Query_thread (chapter 12) */
    mpi_is_thread_main_id,                /**< Id for MPI_Is_thread_main (chapter 12) */
                                          /*
                                           * Chapter 13 I/O
                                           */
    mpi_file_open_id,                     /**< Id for MPI_File_open (chapter 13) */
    mpi_file_close_id,                    /**< Id for MPI_File_close (chapter 13) */
    mpi_file_delete_id,                   /**< Id for MPI_File_delete (chapter 13) */
    mpi_file_set_size_id,                 /**< Id for MPI_File_set_size (chapter 13) */
    mpi_file_preallocate_id,              /**< Id for MPI_File_preallocate (chapter 13) */
    mpi_file_get_size_id,                 /**< Id for MPI_File_get_size (chapter 13) */
    mpi_file_get_group_id,                /**< Id for MPI_File_get_group (chapter 13) */
    mpi_file_get_amode_id,                /**< Id for MPI_File_get_amode (chapter 13) */
    mpi_file_set_info_id,                 /**< Id for MPI_File_set_info (chapter 13) */
    mpi_file_get_info_id,                 /**< Id for MPI_File_get_info (chapter 13) */
    mpi_file_set_view_id,                 /**< Id for MPI_File_set_view (chapter 13) */
    mpi_file_get_view_id,                 /**< Id for MPI_File_get_view (chapter 13) */
    mpi_file_read_at_id,                  /**< Id for MPI_File_read_at (chapter 13) */
    mpi_file_read_at_all_id,              /**< Id for MPI_File_read_at_all (chapter 13) */
    mpi_file_write_at_id,                 /**< Id for MPI_File_write_at (chapter 13) */
    mpi_file_write_at_all_id,             /**< Id for MPI_File_write_at_all (chapter 13) */
    mpi_file_iread_at_id,                 /**< Id for MPI_File_iread_at (chapter 13) */
    mpi_file_iwrite_at_id,                /**< Id for MPI_File_iwrite_at (chapter 13) */
    mpi_file_read_id,                     /**< Id for MPI_File_read (chapter 13) */
    mpi_file_read_all_id,                 /**< Id for MPI_File_read_all (chapter 13) */
    mpi_file_write_id,                    /**< Id for MPI_File_write (chapter 13) */
    mpi_file_write_all_id,                /**< Id for MPI_File_write_all (chapter 13) */
    mpi_file_iread_id,                    /**< Id for MPI_File_iread (chapter 13) */
    mpi_file_iwrite_id,                   /**< Id for MPI_File_iwrite (chapter 13) */
    mpi_file_seek_id,                     /**< Id for MPI_File_seek (chapter 13) */
    mpi_file_get_position_id,             /**< Id for MPI_File_get_position (chapter 13) */
    mpi_file_get_byte_offset_id,          /**< Id for MPI_File_get_byte_offset (chapter 13) */
    mpi_file_read_shared_id,              /**< Id for MPI_File_read_shared (chapter 13) */
    mpi_file_write_shared_id,             /**< Id for MPI_File_write_shared (chapter 13) */
    mpi_file_iread_shared_id,             /**< Id for MPI_File_iread_shared (chapter 13) */
    mpi_file_iwrite_shared_id,            /**< Id for MPI_File_iwrite_shared (chapter 13) */
    mpi_file_read_ordered_id,             /**< Id for MPI_File_read_ordered (chapter 13) */
    mpi_file_write_ordered_id,            /**< Id for MPI_File_write_ordered (chapter 13) */
    mpi_file_seek_shared_id,              /**< Id for MPI_File_seek_shared (chapter 13) */
    mpi_file_get_position_shared_id,      /**< Id for MPI_File_get_position_shared (chapter 13) */
    mpi_file_read_at_all_begin_id,        /**< Id for MPI_File_read_at_all_begin (chapter 13) */
    mpi_file_read_at_all_end_id,          /**< Id for MPI_File_read_at_all_end (chapter 13) */
    mpi_file_write_at_all_begin_id,       /**< Id for MPI_File_write_at_all_begin (chapter 13) */
    mpi_file_write_at_all_end_id,         /**< Id for MPI_File_write_at_all_end (chapter 13) */
    mpi_file_read_all_begin_id,           /**< Id for MPI_File_read_all_begin (chapter 13) */
    mpi_file_read_all_end_id,             /**< Id for MPI_File_read_all_end (chapter 13) */
    mpi_file_write_all_begin_id,          /**< Id for MPI_File_write_all_begin (chapter 13) */
    mpi_file_write_all_end_id,            /**< Id for MPI_File_write_all_end (chapter 13) */
    mpi_file_read_ordered_begin_id,       /**< Id for MPI_File_read_ordered_begin (chapter 13) */
    mpi_file_read_ordered_end_id,         /**< Id for MPI_File_read_ordered_end (chapter 13) */
    mpi_file_write_ordered_begin_id,      /**< Id for MPI_File_write_ordered_begin (chapter 13) */
    mpi_file_write_ordered_end_id,        /**< Id for MPI_File_write_ordered_end (chapter 13) */
    mpi_file_get_type_extent_id,          /**< Id for MPI_File_get_type_extent (chapter 13) */
    mpi_register_datarep_id,              /**< Id for MPI_Register_datarep (chapter 13) */
    mpi_file_set_atomicity_id,            /**< Id for MPI_File_set_atomicity (chapter 13) */
    mpi_file_get_atomicity_id,            /**< Id for MPI_File_get_atomicity (chapter 13) */
    mpi_file_sync_id,                     /**< Id for MPI_File_sync (chapter 13) */
                                          /*
                                           * Chapter 15 Deprecated Functions
                                           */
    mpi_keyval_create_id,                 /**< Id for MPI_Keyval_create (chapter 15) */
    mpi_keyval_free_id,                   /**< Id for MPI_Keyval_free (chapter 15) */
    mpi_attr_put_id,                      /**< Id for MPI_Attr_put (chapter 15) */
    mpi_attr_get_id,                      /**< Id for MPI_Attr_get (chapter 15) */
    mpi_attr_delete_id,                   /**< Id for MPI_Attr_delete (chapter 15) */
                                          /*
                                           * Chapter 16 Removed Interfaces
                                           *
                                           * Chapter 17 Language Bindings
                                           */
    mpi_type_create_f90_real_id,          /**< Id for MPI_Type_create_f90_real (chapter 17) */
    mpi_type_create_f90_complex_id,       /**< Id for MPI_Type_create_f90_complex (chapter 17) */
    mpi_type_create_f90_integer_id,       /**< Id for MPI_Type_create_f90_integer (chapter 17) */
    mpi_type_match_size_id,               /**< Id for MPI_Type_match_size (chapter 17) */
    mpi_status_f2c_id,                    /**< Id for MPI_Status_f2c (chapter 17) */
    mpi_status_c2f_id,                    /**< Id for MPI_Status_c2f (chapter 17) */
    mpi_status_f082c_id,                  /**< Id for MPI_Status_f082c (chapter 17) */
    mpi_status_c2f08_id,                  /**< Id for MPI_Status_c2f08 (chapter 17) */
    mpi_status_f2f08_id,                  /**< Id for MPI_Status_f2f08 (chapter 17) */
    mpi_status_f082f_id,                  /**< Id for MPI_Status_f082f (chapter 17) */
    number_mpicall_ids,                   /**< Number of MPI_calls */
                                          /*
                                           * Identifiers for load imbalance detection
                                           */
    mpi_barrier_barrier_id,               /**< Id for MPI_Barrier_barrier (load imbalance) */
    mpi_bcast_barrier_id,                 /**< Id for MPI_Bcast_barrier (load imbalance) */
    mpi_gather_barrier_id,                /**< Id for MPI_Gather_barrier (load imbalance) */
    mpi_gatherv_barrier_id,               /**< Id for MPI_Gatherv_barrier (load imbalance) */
    mpi_scatter_barrier_id,               /**< Id for MPI_Scatter_barrier (load imbalance) */
    mpi_scatterv_barrier_id,              /**< Id for MPI_Scatterv_barrier (load imbalance) */
    mpi_allgather_barrier_id,             /**< Id for MPI_Allgather_barrier (load imbalance) */
    mpi_allgatherv_barrier_id,            /**< Id for MPI_Allgatherv_barrier (load imbalance) */
    mpi_alltoall_barrier_id,              /**< Id for MPI_Alltoall_barrier (load imbalance) */
    mpi_alltoallv_barrier_id,             /**< Id for MPI_Alltoallv_barrier (load imbalance) */
    mpi_alltoallw_barrier_id,             /**< Id for MPI_Alltoallw_barrier (load imbalance) */
    mpi_reduce_barrier_id,                /**< Id for MPI_Reduce_barrier (load imbalance) */
    mpi_allreduce_barrier_id,             /**< Id for MPI_Allreduce_barrier (load imbalance) */
    mpi_reduce_scatter_block_barrier_id,  /**< Id for MPI_Reduce_scatter_block_barrier (load imbalance) */
    mpi_reduce_scatter_barrier_id,        /**< Id for MPI_Reduce_scatter_barrier (load imbalance) */
    mpi_scan_barrier_id,                  /**< Id for MPI_Scan_barrier (load imbalance) */
    mpi_exscan_barrier_id,                /**< Id for MPI_Exscan_barrier (load imbalance) */
    mpi_ibarrier_barrier_id,              /**< Id for MPI_Ibarrier_barrier (load imbalance) */
    mpi_ibcast_barrier_id,                /**< Id for MPI_Ibcast_barrier (load imbalance) */
    mpi_igather_barrier_id,               /**< Id for MPI_Igather_barrier (load imbalance) */
    mpi_igatherv_barrier_id,              /**< Id for MPI_Igatherv_barrier (load imbalance) */
    mpi_iscatter_barrier_id,              /**< Id for MPI_Iscatter_barrier (load imbalance) */
    mpi_iscatterv_barrier_id,             /**< Id for MPI_Iscatterv_barrier (load imbalance) */
    mpi_iallgather_barrier_id,            /**< Id for MPI_Iallgather_barrier (load imbalance) */
    mpi_iallgatherv_barrier_id,           /**< Id for MPI_Iallgatherv_barrier (load imbalance) */
    mpi_ialltoall_barrier_id,             /**< Id for MPI_Ialltoall_barrier (load imbalance) */
    mpi_ialltoallv_barrier_id,            /**< Id for MPI_Ialltoallv_barrier (load imbalance) */
    mpi_ialltoallw_barrier_id,            /**< Id for MPI_Ialltoallw_barrier (load imbalance) */
    mpi_ireduce_barrier_id,               /**< Id for MPI_Ireduce_barrier (load imbalance) */
    mpi_iallreduce_barrier_id,            /**< Id for MPI_Iallreduce_barrier (load imbalance) */
    mpi_ireduce_scatter_block_barrier_id, /**< Id for MPI_Ireduce_scatter_block_barrier (load imbalance) */
    mpi_ireduce_scatter_barrier_id,       /**< Id for MPI_Ireduce_scatter_barrier (load imbalance) */
    mpi_iscan_barrier_id,                 /**< Id for MPI_Iscan_barrier (load imbalance) */
    mpi_iexscan_barrier_id,               /**< Id for MPI_Iexscan_barrier (load imbalance) */
    number_mpi_ids                        /**< Number of such identifiers */
} LMX_MPItrace_id;

#endif /* SRC_FEATURES_BASIC_LMX_MPITRACE_IDS_H_ */
